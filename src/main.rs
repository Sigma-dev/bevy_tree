use bevy::{
    gltf::GltfMaterialName,
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    scene::SceneInstanceReady,
};
use bevy_editor_cam::{prelude::EditorCam, DefaultEditorCamPlugins};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultEditorCamPlugins,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, LeafMaterial>>::default(),
        ))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        EditorCam::default(),
        Transform::from_translation(Vec3::ONE * 30.).looking_at(Vec3::Y * 20., Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 2000.,
            ..default()
        },
        Transform::default().looking_at(-Vec3::ONE, Vec3::Y),
    ));

    commands
        .spawn(SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.glb")),
        ))
        .observe(replace_materials);
}

fn replace_materials(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    standard_material_handles: Query<(
        Entity,
        &MeshMaterial3d<StandardMaterial>,
        &GltfMaterialName,
    )>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut leaf_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, LeafMaterial>>>,
) {
    let root = trigger.entity();
    for (mesh, handle, GltfMaterialName(name)) in children
        .iter_descendants(root)
        .filter_map(|e| standard_material_handles.get(e).ok())
    {
        println!("{:?}", name);
        let mut replace = false;
        match name.as_str() {
            "M_Leaf_01.004" => {
                replace = true;
                commands
                    .entity(mesh)
                    .insert(MeshMaterial3d(leaf_materials.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color: Color::WHITE,
                            opaque_render_method: OpaqueRendererMethod::Auto,
                            alpha_mode: AlphaMode::Opaque,
                            cull_mode: None,
                            reflectance: 0.,
                            metallic: 0.,
                            perceptual_roughness: 0.,
                            diffuse_transmission: 0.6,
                            ..Default::default()
                        },
                        extension: LeafMaterial {
                            color: Color::srgb(0.2, 0.6, 0.2).into(),
                            color_texture: asset_server.load("T_Leaf_01.png"),
                            noise_texture: asset_server.load("T_Noise.png"),
                            alpha_mode: AlphaMode::Opaque,
                        },
                    })));
            }
            "M_Bark_05" => {
                commands
                    .entity(mesh)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("T_Bark_Jagged_Albedo.png")),
                        base_color: Color::srgb_from_array([0.8; 3]),
                        reflectance: 0.,
                        metallic: 0.,
                        perceptual_roughness: 0.,
                        ..default()
                    })));
            }
            _ => {}
        }
        if replace {
            commands
                .entity(mesh)
                .remove::<MeshMaterial3d<StandardMaterial>>();
        }
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct LeafMaterial {
    #[uniform(100)]
    color: LinearRgba,
    #[texture(101)]
    #[sampler(102)]
    color_texture: Handle<Image>,
    #[texture(103)]
    #[sampler(104)]
    noise_texture: Handle<Image>,
    alpha_mode: AlphaMode,
}

impl MaterialExtension for LeafMaterial {
    fn fragment_shader() -> ShaderRef {
        "leaf.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "leaf.wgsl".into()
    }
}
