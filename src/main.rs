use bevy::prelude::*;
use clap::Parser;
use once_cell::sync::Lazy;

mod lib_parser;
mod static_lib;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = "lib/gojuon.toml")]
    path: String,
}

static ARGS: Lazy<Args> = Lazy::new(|| Args::parse());

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component, Debug)]
struct Data { // count & lib
    current_word_index: i32, // -1: not chosen, >=0: index of the library
    words_lib: Vec<(String, String)>,
}

impl Data {
    fn new(
        current_word_index: i32,
        words_lib: Vec<(String, String)>,
    ) -> Self {
        Data {
            current_word_index,
            words_lib,
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    // window
    let mut window = windows.single_mut();
    window.resolution.set(800.0, 500.0);

    // camera
    commands.spawn(Camera2dBundle::default());

    // text
    let font = asset_server.load("NotoSansJP-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 175.0,
        ..Default::default()
    };
    let text_justification = JustifyText::Center;

    let word_lib = match lib_parser::load_library_from_csv(&ARGS.path) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("\n读取词库文件时发生错误，请检查词库 \"{}\" 是否存在！\n读取错误信息：{}\n", ARGS.path, e);
            std::process::exit(1);
        }
    };

    let data = Data::new(
        -1,
        word_lib
    );

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Press Space", text_style.clone())
                .with_justify(text_justification),
            ..Default::default()
        },
        data,
    ));
}

fn update(
    kb: Res<ButtonInput<KeyCode>>,
    mut x: Query<(&mut Text, &mut Data)>,
) {
    if kb.just_pressed(KeyCode::Space) != true {
        return;
    }
    
    let mut y = x.iter_mut().next().unwrap();

    let (text, data) = (&mut y.0, &mut y.1);

    if data.current_word_index == -1 {
        let v = rand::random::<usize>() % data.words_lib.len();
        data.current_word_index = v as i32;
        text.sections[0].value = data.words_lib[v as usize].0.to_string();
    } else {
        let v = data.current_word_index;
        data.current_word_index = -1;
        text.sections[0].value = format!("{} {}", data.words_lib[v as usize].0, data.words_lib[v as usize].1);
    }
}
