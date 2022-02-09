use bevy::prelude::*;

fn main_title_spawn(
    mut commands: Commands,
    windows: Res<Windows>,
    main_title_res: Res<super::states::MainTitleRes>,
) {
    let windows = windows.get_primary().unwrap();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(windows.physical_width() as f32),
                    height: Val::Px(windows.physical_height() as f32),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(super::states::UIForStat(vec![
            super::states::MainTitleState::Main,
        ]))
        .with_children(|p| {
            p.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(windows.physical_width() as f32),
                        height: Val::Px(windows.physical_height() as f32),
                    },
                    ..Default::default()
                },
                image: main_title_res.bg_image.clone().into(),
                ..Default::default()
            });
        });
}
