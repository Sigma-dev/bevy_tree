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
            MaterialPlugin::<CustomMaterial>::default(),
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, GroundMaterial>>::default(),
        ))
        .add_systems(Startup, setup)
        .run()
}

#[derive(Resource)]
struct TreeAsset(Handle<Gltf>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3d::default(), EditorCam::default()));

    commands.spawn((
        DirectionalLight {
            illuminance: 2000.,
            ..default()
        },
        Transform::default().looking_at(-Vec3::ONE, Vec3::Y),
    ));

    let tree = TreeAsset(asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.glb")));

    commands
        .spawn(SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.glb")),
        ))
        .observe(replace_materials);

    commands.insert_resource(tree);
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
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut ground2_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, GroundMaterial>>>,
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
                /*  commands
                .entity(mesh)
                .insert(MeshMaterial3d(custom_materials.add(CustomMaterial {
                    color: Color::srgb(0.1, 0.4, 0.1).into(),
                    color_texture: asset_server.load("T_Leaf_01.png"),
                    alpha_mode: AlphaMode::Blend,
                }))); */
                commands
                    .entity(mesh)
                    .insert(MeshMaterial3d(ground2_materials.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color: Color::WHITE,
                            opaque_render_method: OpaqueRendererMethod::Auto,
                            alpha_mode: AlphaMode::Opaque,
                            cull_mode: None,
                            ..Default::default()
                        },
                        extension: GroundMaterial {
                            color: Color::srgb(0.1, 0.4, 0.1).into(),
                            color_texture: asset_server.load("T_Leaf_01.png"),
                            alpha_mode: AlphaMode::Opaque,
                        },
                    })));
            }
            "M_Bark_05" => {
                commands
                    .entity(mesh)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("T_Bark_Jagged_Albedo.png")),
                        reflectance: 0.,
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "leaf.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct GroundMaterial {
    #[uniform(100)]
    color: LinearRgba,
    #[texture(101)]
    #[sampler(102)]
    color_texture: Handle<Image>,
    alpha_mode: AlphaMode,
}

impl MaterialExtension for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "ground.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "ground.wgsl".into()
    }
}
