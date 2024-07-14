use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use rapier2d::{parry::query, prelude::*};
use gloo_console::log;
use crate::utils::Point;
use crate::{GAME_WIDTH, GAME_HEIGHT};

pub struct RopeManager{
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    ball_handles: Vec::<RigidBodyHandle>,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    hooks: (),
    events: (),
    physics_pipeline: PhysicsPipeline
}

const MAX_JOINT_SET: u32 = 20;

impl RopeManager {
    pub fn new() -> Self {

        let mut rigid_body_set: RigidBodySet = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let mut balls = Vec::<RigidBodyHandle>::new();


        let mut prev_handle: Option<RigidBodyHandle> = None;
        let multibody_joint_set = MultibodyJointSet::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let x_diff = 10.0;
        let y_diff = -30.0;
        let mut x = 400.0;
        let mut y = 100.0;
        for i in 0..MAX_JOINT_SET {
            let rigid_body = if i == 0 {
                RigidBodyBuilder::fixed()
                    .translation(vector![x, y])
                    .build()
            } else if i == MAX_JOINT_SET - 1 {
                // Last one
                x += x_diff;
                y -= y_diff;
                RigidBodyBuilder::dynamic()
                    .translation(vector![x, y])
                    .linear_damping(0.1)
                    .additional_mass(0.5)
                    // .lock_rotations()
                    .build()
            }else if i < MAX_JOINT_SET / 2 {
                x += x_diff;
                y -= y_diff;
                RigidBodyBuilder::dynamic() 
                    .translation(vector![x, y])
                    .linear_damping(0.1)
                    .additional_mass(0.5)
                    .build()
            } else {
                let half_num = MAX_JOINT_SET / 2;
                let apex = 100.0 + (half_num as f32 * 15.0);
                let count = i - half_num;
                x += x_diff;
                y -= y_diff;

                RigidBodyBuilder::dynamic() 
                    .translation(vector![x, y])
                    .linear_damping(0.1)
                    .additional_mass(0.5)
                    .build()
            };
            let collider = ColliderBuilder::cuboid(10.0, 0.1).restitution(0.1).build();
            let ball_body_handle = rigid_body_set.insert(rigid_body);
            collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
            balls.push(ball_body_handle);


            match prev_handle {
                Some(hand) => {
                    // let joint = RopeJointBuilder::new(0.0)
                    //     .local_anchor1(point![0.0, 0.0])
                    //     .local_anchor2(point![0.0, y_diff]);
                    
                    // let joint = RevoluteJointBuilder::new()
                    //     .local_anchor1(point![0.0, 0.0])
                    //     .local_anchor2(point![0.0, 0.0]);

                    let joint = FixedJointBuilder::new()
                        .local_anchor1(point![0.0, 0.0])
                        .local_anchor2(point![0.0, y_diff]);

                    // let x = Vector::x_axis();
                    // let mut joint = PrismaticJointBuilder::new(x)
                    //     .local_anchor1(point![0.0, 0.0])
                    //     .local_anchor2(point![0.8, 0.8]);
                    //     .limits([-2.0, 5.0]);

                    // let mut joint = FixedJointBuilder::new()
                    //     .local_anchor1(point![x, y])
                    //     .local_anchor2(point![10.0, 10.0]);

                    impulse_joint_set.insert(hand, ball_body_handle, joint, true);
                
                }, 
                None => {}
            }
            prev_handle = Some(ball_body_handle)
        }
       
        // let rigid_body = RigidBodyBuilder::dynamic()
        //     .translation(vector![2.0,9.0])
        //     .build();
        // let collider = ColliderBuilder::ball(0.5).restitution(1.75).build();
        // let ball_body_handle2 = rigid_body_set.insert(rigid_body);
        // collider_set.insert_with_parent(collider, ball_body_handle2, &mut rigid_body_set);

    
        let gravity = vector![0.0, 9.81];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();

        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        RopeManager {
            rigid_body_set: rigid_body_set,
            collider_set: collider_set,
            ball_handles: balls,
            gravity: gravity,
            integration_parameters: integration_parameters,
            islands: island_manager,
            broad_phase: broad_phase,
            narrow_phase: narrow_phase,
            impulse_joints: impulse_joint_set,
            multibody_joints: multibody_joint_set,
            ccd_solver: ccd_solver,
            query_pipeline: query_pipeline,
            hooks: (),
            events: (),
            physics_pipeline: physics_pipeline
        }
    }
    
    pub fn update(&mut self, delta: f64) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.hooks,
            &self.events,
        );
    }
    
    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        // Draw floor
        ctx.set_fill_style(&JsValue::from("rgb(55, 255, 55)"));  
        // Draw balls

        let mut prev_ball_pos: Option<Point::<f64>> = None;
        for bh in self.ball_handles.iter() {
            ctx.set_fill_style(&JsValue::from("rgb(255, 255, 55)"));
            let ball_body: &RigidBody = &self.rigid_body_set[*bh];

            let x = ball_body.translation().x as f64;
            let y = ball_body.translation().y as f64;
            let _ = ctx.fill_rect(x - 10.0, y-2.5, 20.0, 5.0);
        
            match prev_ball_pos {
                Some(pos) => {
                    ctx.set_stroke_style(&JsValue::from("rgb(255, 55, 55)"));
                    let _ = ctx.begin_path();
                    let _ = ctx.move_to(pos.x, pos.y);
                    let _ = ctx.line_to(ball_body.translation().x as f64, ball_body.translation().y as f64);

                    let _ = ctx.stroke();
                }, 
                None => {}
            }

            ctx.set_stroke_style(&JsValue::from("rgb(55, 255, 55)"));
            let rot = ball_body.rotation().angle() as f64;
            
            ctx.set_stroke_style(&JsValue::from("rgb(55, 255, 55)"));
            let _ = ctx.begin_path();
            let _ = ctx.move_to(ball_body.translation().x as f64, ball_body.translation().y as f64);
            let _ = ctx.line_to(
                ball_body.translation().x as f64 + (20.0 * rot.sin()), 
                ball_body.translation().y as f64 - (20.0 * rot.cos())
            );
            let _ = ctx.stroke();
            prev_ball_pos = Some(Point::new(ball_body.translation().x as f64, ball_body.translation().y as f64));
        }
    }

    pub fn set_ball_pos(&mut self, pt: Point::<f32>) {
        let mut ball_body = self.rigid_body_set.get_mut(self.ball_handles[MAX_JOINT_SET as usize - 1]).unwrap();
        ball_body.set_translation(vector![pt.x, pt.y], true);
        ball_body.set_linvel(vector![0.0, 0.0], true);
        ball_body.set_angvel(0.0, true);

        for bb in self.ball_handles.iter_mut() {
            let brbs = self.rigid_body_set.get_mut(*bb);
            match brbs {
                Some(brbs) => {
                    brbs.set_linvel(vector![0.0, 0.0], true);
                    brbs.set_angvel(0.0, true);
                },
                None => {}
            }     
        }

    }
    
}