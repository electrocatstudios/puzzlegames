use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use rapier2d::{parry::query, prelude::*};
use gloo_console::log;
use crate::utils::Point;

pub struct RopeManager{
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    ball_body_handle: RigidBodyHandle,
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

const SCALE: f64 = 50.0;

impl RopeManager {
    pub fn new() -> Self {

        let mut rigid_body_set: RigidBodySet = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert(collider);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(1.75).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        RopeManager {
            rigid_body_set: rigid_body_set,
            collider_set: collider_set,
            ball_body_handle: ball_body_handle,
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
        let ball_body = &self.rigid_body_set[self.ball_body_handle];

        ctx.set_fill_style(&JsValue::from("rgb(55, 255, 55)"));
        let half_box_width = (SCALE * 2.0) / 2.0;
        ctx.fill_rect((SCALE * 10.0) - half_box_width, SCALE * 10.0, SCALE * 2.0, SCALE * 0.1);
        
        ctx.set_fill_style(&JsValue::from("rgb(255, 255, 55)"));

        let _ = ctx.begin_path();
        let _ = ctx.arc(
                    SCALE * 10.0,
                    (10.0 * SCALE) - SCALE * ball_body.translation().y as f64,
                    SCALE * 0.5, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();
    }

    pub fn set_ball_pos(&mut self, pt: Point::<f32>) {
        let mut ball_body = self.rigid_body_set.get_mut(self.ball_body_handle).unwrap();
        ball_body.set_translation(vector![0.0, pt.y], true);
        ball_body.set_linvel(vector![0.0, 0.0], true);
        ball_body.set_angvel(0.0, true);
    }
}