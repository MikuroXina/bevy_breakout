use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::game::exp::{Ball, Collider, Scoreboard};

pub fn system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (mut ball, ball_trans, ball_sprite) in ball_query.iter_mut() {
        let ball_size = ball_sprite.size;
        let velocity = &mut ball.velocity;

        for (entity, collider, col_trans, col_sprite) in collider_query.iter() {
            let collision = collide(
                ball_trans.translation,
                ball_size,
                col_trans.translation,
                col_sprite.size,
            );
            if let Some(collision) = collision {
                let reflect_x = match collision {
                    Collision::Left => 0. < velocity.x(),
                    Collision::Right => velocity.x() < 0.,
                    _ => false,
                };
                let reflect_y = match collision {
                    Collision::Top => velocity.y() < 0.,
                    Collision::Bottom => 0. < velocity.y(),
                    _ => false,
                };

                if reflect_x {
                    *velocity.x_mut() *= -1.;
                }
                if reflect_y {
                    *velocity.y_mut() *= -1.;
                }

                match collider {
                    &Collider::Scorable => {
                        scoreboard.score += 1;
                        commands.despawn(entity);
                    }
                    &Collider::Solid => break,
                    _ => {}
                }
            }
        }
    }
}
