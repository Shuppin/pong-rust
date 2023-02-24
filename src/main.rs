use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::event;
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};

const PADDING: f32 = 40.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const BALL_SIZE: f32 = 30.0;
const PLAYER_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 500.0;
const COLOUR_GREY: graphics::Color = graphics::Color::new(0.3, 0.3, 0.3, 1.0);

fn clamp(value: &mut f32, low: f32, high: f32) {
    // *value de-references value, getting the actual f32
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

// Updates the paddle position
fn move_paddle(position: &mut na::Point2<f32>, keycode: KeyCode, y_direction: f32, ctx: &mut Context) {

    let dt = ggez::timer::delta(ctx);
    let screen_height = graphics::drawable_size(ctx).1;

    if keyboard::is_key_pressed(ctx, keycode) {
        position.y += y_direction * PLAYER_SPEED * dt.as_secs_f32();
    }

    clamp(&mut position.y, PADDLE_HEIGHT*0.5, screen_height-(PADDLE_HEIGHT*0.5));

}

// Flips a given vector in the x and y randomly
fn randomise_vector(vector: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vector.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vector.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_position: na::Point2<f32>,
    player_2_position: na::Point2<f32>,
    player_1_score: i32,
    player_2_score: i32,
    ball_position: na::Point2<f32>,
    ball_velocity: na::Vector2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        // Initialise a random start velocity
        let mut ball_velocity = na::Vector2::new(0.0, 0.0);
        randomise_vector(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_position: na::Point2::new((PADDLE_WIDTH*0.5)+PADDING, screen_height*0.5),
            player_2_position: na::Point2::new(screen_width-(PADDLE_WIDTH*0.5)-PADDING, screen_height*0.5),
            player_1_score: 0,
            player_2_score: 0,
            ball_position: na::Point2::new(screen_width*0.5, screen_height*0.5),
            ball_velocity
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        // Update paddle positions if their key is held down
        move_paddle(&mut self.player_1_position, KeyCode::W, -1.0, ctx);
        move_paddle(&mut self.player_1_position, KeyCode::S, 1.0, ctx);
        move_paddle(&mut self.player_2_position, KeyCode::Up, -1.0, ctx);
        move_paddle(&mut self.player_2_position, KeyCode::Down, 1.0, ctx);

        self.ball_position += self.ball_velocity * dt;

        // Scoring and reseting ball position
        if self.ball_position.x < 0.0 {
            self.ball_position.x = screen_width*0.5;
            self.ball_position.y = screen_height*0.5;
            randomise_vector(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        } else if self.ball_position.x > screen_width {
            self.ball_position.x = screen_width*0.5;
            self.ball_position.y = screen_height*0.5;
            randomise_vector(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }

        // Edge bounces
        if self.ball_position.y < BALL_SIZE*0.5{
            self.ball_position.y = BALL_SIZE*0.5;
            self.ball_velocity.y = self.ball_velocity.y.abs();
        } else if self.ball_position.y > screen_height - (BALL_SIZE*0.5) {
            self.ball_position.y = screen_height - (BALL_SIZE*0.5);
            self.ball_velocity.y = -self.ball_velocity.y.abs();
        }

        // Collsion conditions
        let touching_player_1 =
            self.ball_position.x - (BALL_SIZE*0.5) < self.player_1_position.x + (PADDLE_WIDTH*0.5)
            && self.ball_position.x + (BALL_SIZE*0.5) > self.player_1_position.x - (PADDLE_WIDTH*0.5)
            && self.ball_position.y - (BALL_SIZE*0.5) < self.player_1_position.y + (PADDLE_HEIGHT*0.5)
            && self.ball_position.y + (BALL_SIZE*0.5) > self.player_1_position.y - (PADDLE_HEIGHT*0.5);

        let touching_player_2 =
            self.ball_position.x - (BALL_SIZE*0.5) < self.player_2_position.x + (PADDLE_WIDTH*0.5)
            && self.ball_position.x + (BALL_SIZE*0.5) > self.player_2_position.x - (PADDLE_WIDTH*0.5)
            && self.ball_position.y - (BALL_SIZE*0.5) < self.player_2_position.y + (PADDLE_HEIGHT*0.5)
            && self.ball_position.y + (BALL_SIZE*0.5) > self.player_2_position.y - (PADDLE_HEIGHT*0.5);

        // Invert ball velocity if touching a player paddle
        if touching_player_1 {
            self.ball_velocity.x = self.ball_velocity.x.abs();
        } else if touching_player_2 {
            self.ball_velocity.x = -self.ball_velocity.x.abs();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen
        graphics::clear(ctx, graphics::BLACK);

        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        // Create game objects
        let paddle_rect = graphics::Rect::new(
            -PADDLE_WIDTH * 0.5,
            -PADDLE_HEIGHT * 0.5,
            PADDLE_WIDTH,
            PADDLE_HEIGHT
        );
        let paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            paddle_rect,
            graphics::WHITE
        )?;

        let ball_rect = graphics::Rect::new(
            -BALL_SIZE*0.5, 
            -BALL_SIZE*0.5, 
            BALL_SIZE, 
            BALL_SIZE
        );
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE
        )?;

        let seperating_line_mesh = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(screen_width*0.5, 0.0+PADDING),
                na::Point2::new(screen_width*0.5,screen_height-PADDING)
            ],
            2.0,
            COLOUR_GREY
        )?;

        // Draw game objects
        let mut draw_param = graphics::DrawParam::default();

        graphics::draw(ctx, &seperating_line_mesh, draw_param)?;

        draw_param.dest = self.player_1_position.into();
        graphics::draw(ctx, &paddle_mesh, draw_param)?;

        draw_param.dest = self.player_2_position.into();
        graphics::draw(ctx, &paddle_mesh, draw_param)?;
        
        draw_param.dest = self.ball_position.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;
        
        // Draw score
        let score_text = graphics::Text::new(format!("{}        {}", self.player_1_score, self.player_2_score));

        let mut score_position = na::Point2::new(screen_width*0.5, 0.0);
        let (score_text_width, score_text_height) = score_text.dimensions(ctx);
        score_position -= na::Vector2::new(score_text_width as f32 * 0.5, score_text_height as f32 * -0.5);
        draw_param.dest = score_position.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        // Draw fps counter
        let fps = ggez::timer::fps(ctx);
        let fps_text = graphics::Text::new(format!("{:.2} FPS", fps));
        draw_param.dest = na::Point2::new(0.0, 0.0).into();

        graphics::draw(ctx, &fps_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())

    }
}

fn main() -> GameResult {
    // Set up the window with default settings and turn off vsync
    let window_setup = ggez::conf::WindowSetup::default().vsync(false);
    // Set up the context builder with the window setup
    let cb = ggez::ContextBuilder::new("phys_1", "me")
        .window_setup(window_setup);
    // Build the context and event loop
    let (ctx, event_loop) = &mut cb.build()?;
    // Set the window title
    graphics::set_window_title(ctx, "phys_1");
    // Create a new instance of the MainState struct
    let mut state = MainState::new(ctx);
    // Run the event loop with the state object
    event::run(ctx, event_loop, &mut state)?;
    // Return an Ok() result if the program finishes successfully
    Ok(())
}
