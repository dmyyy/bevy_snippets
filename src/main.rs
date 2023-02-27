/*
Bevy Camera that looks in the direction of your cursor
pub fn orbit_camera(
    mut q_camera: Query<(&mut Transform, &mut Rotation), With<PlayerCamera>>,
    mut motion_evr: EventReader<MouseMotion>,
    windows: Res<Windows>,
) {
    let (mut transform, mut rotation) = q_camera.single_mut();

    let mut rotation_move = Vec2::ZERO;
    for ev in motion_evr.iter() {
        rotation_move += ev.delta;
    }

    if rotation_move.x.powi(2) + rotation_move.y.powi(2) > 0. {
        let window = windows.get_primary().expect("no primary window found!");
        let delta_x = (rotation_move.x / window.width()) * TWO_PI * HORIZONTAL_ROTATION_SENSITIVITY;
        let delta_y = (rotation_move.y / window.height()) * PI * VERTICAL_ROTATION_SENSITIVITY;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        transform.rotation = yaw * transform.rotation; // rotate around global y axis
        transform.rotation = transform.rotation * pitch; // rotate around local x axis
    }
}
*/
