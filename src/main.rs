use bevy::prelude::*;
use clap::Parser;

mod lib_parser;

#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = "lib/gojuon.toml")]
    path: String,
}

fn main() {
    let args = Args::parse();
    App::new()
        .insert_resource(args)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component, Debug)]
struct Data { // count & lib
    current_word_index: i32, // -1 表示未选中，>=0 表示词库索引
    words_lib: Vec<(String, String, String)>, // (词, 读音, 释义)
}

impl Data {
    fn new(
        current_word_index: i32,
        words_lib: Vec<(String, String, String)>,
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
    args: Res<Args>,
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

    let word_lib = match lib_parser::load_library_from_toml(&args.path) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("\n读取词库文件时发生错误，请检查词库 \"{}\" 是否存在！\n读取错误信息：{}\n", &args.path, e);
            std::process::exit(1);
        }
    };

    let data = Data::new(
        -1,
        word_lib
    );

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Press Space", text_style)
                .with_justify(JustifyText::Center),
            ..Default::default()
        },
        data,
    ));
}

fn update(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Text, &mut Data)>,
) {
    if !kb.just_pressed(KeyCode::Space) {
        return;
    }
    
    // let mut y = query.iter_mut().next().unwrap();
    // let (text, data) = (&mut y.0, &mut y.1);
    let (mut text, mut data) = query.single_mut();

    text.sections[0].value = if data.current_word_index == -1 {
        let idx = rand::random::<usize>() % data.words_lib.len();
        data.current_word_index = idx as i32;

        let word = &data.words_lib[idx as usize];

        format!("{}\n{}\n{}", word.0, " ", word.2)
    } else {
        let idx = data.current_word_index;
        data.current_word_index = -1;
        
        let word = &data.words_lib[idx as usize];

        format!("{}\n{}\n{}", word.0, word.1, word.2)
    };

}
