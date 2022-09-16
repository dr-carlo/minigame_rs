use macroquad::prelude::*;

// COPIA DE 'juego1-JS', 
// EJECUTAR 'cargo run' Y 
// USAR FLECHAS PARA JUGAR !
const CANT_PF: i8 = 3;
pub enum GameState { 
    Game, Dead,
}

struct Platform { 
    plataformas: Vec<Rect> 
}

impl Platform {
    pub fn new() -> Platform {
        Platform {plataformas: posiciones(CANT_PF)}
    }

    pub fn draw(&self) { 
        for i in 0..3 {
            draw_rectangle( self.plataformas[i].x, self.plataformas[i].y, 
                self.plataformas[i].w, self.plataformas[i].h, DARKGRAY);
        };
    }

    pub fn update(&mut self, dt: f32, v: f32) {
        for i in 0..3 {
            self.plataformas[i].y += v * dt;
        }
    }
}

pub struct BaN {
    ban: Rect
}

impl BaN {
    pub fn new() -> Self {
        Self {
            ban: Rect::new(screen_width() * 0.5 , 0.5, 15.0, 15.0), 
        }
    }

    pub fn update(&mut self, dt: f32, v: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -555.5, // velocidad
            (false, true) => 555.5, 
            _ => 0.0,
        }; 
        self.ban.x += x_move * dt;  

        if self.ban.x < 15.0 {self.ban.x = 23.0;} 
        if self.ban.x > screen_width() - self.ban.w { 
            self.ban.x = screen_width() - self.ban.w; 
        } 

        let y_move = match is_key_down(KeyCode::Down) {
            true => v * 2.5, 
            false => v
        };

        self.ban.y += dt * y_move; 

    }

    pub fn draw(&self) {  
        draw_circle(self.ban.x, self.ban.y, self.ban.h, ORANGE);
        let g = Circle::new(self.ban.x - 5.5, self.ban.y -5.5, self.ban.h * 0.9);
        draw_circle(g.x, g.y, g.r, BLUE); // circulo sobrepuesto    
    }
}

pub fn posiciones(n: i8) -> Vec<Rect> {
    let mut plataformas = Vec::new();
    for _ in 0..n { // cantidad de plataformas
        let r: f32 = rand::gen_range(0.1, 0.8);
        let z = Rect {
            x: screen_width() * r, 
            y: screen_height() * r, 
            w: 155.5, 
            h: 22.2,
        };
        plataformas.push(z);
    }
    plataformas
}

fn detenciones(a: &mut Rect, b: &Rect) -> bool { 
        if a.overlaps(&b) {
            a.y = b.y - 14.0;
            return true
        }
        false 
}

fn new_game(
    score: &mut i32, lives: &mut i32, 
    bn: &mut BaN, ptf_speed: &mut f32, ban_speed: &mut f32 ) {
    *score = 0; *lives = 5; *bn = BaN::new();
    *ptf_speed = -55.5; *ban_speed = 155.5;
}

#[macroquad::main("breakout")]
async fn main() {
    let mut game_state = GameState::Game;
    let mut bn = BaN::new();
    let mut ptf = Platform::new();
    let font = load_ttf_font("Res/Heebo-VariableFont_wght.ttf").await.unwrap();
    let mut score = 0;
    let mut lives = 5;
    let mut ptf_speed: f32 = -55.5;
    let mut ban_speed: f32 = 155.5;

    loop {
        match game_state {
            GameState::Game => { 
                ban_speed += 0.05;
                bn.update(get_frame_time(), ban_speed); 

                ptf_speed -= 0.3;
                ptf.update(get_frame_time(), ptf_speed);

                for i in 0..3 {
                    if detenciones(&mut bn.ban, &ptf.plataformas[i]) {
                        score += 1;
                    }
                }
            }
            GameState::Dead => {  
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    new_game(
                        &mut score, &mut lives, &mut bn,
                        &mut ptf_speed, &mut ban_speed,
                    );
                }
            }
        }
        
        for i in 0..3 {
            if ptf.plataformas[i].y < screen_height() * 0.01 {
                let a: f32 = rand::gen_range(0.05, 0.85); 
                ptf.plataformas[i].x = screen_width() * a;
                ptf.plataformas[i].y = screen_height();
            }
        }

        if bn.ban.y < screen_height() - (screen_height() + 0.55) {
            bn.ban.y = screen_height() * 0.1; 
            if lives > 1 {
                lives -= 1;
            } else {
                game_state = GameState::Dead;
            }
        }

        if bn.ban.y > (screen_height() + 0.55) { 
            bn.ban.y = screen_height() * 0.1; 
            if lives > 1 {
                lives -= 1;
            } else {
                game_state = GameState::Dead;
            }
        } 
        // color de fondo 
        clear_background(BLUE); 
        bn.draw();
        ptf.draw();

        match game_state {
            GameState::Game => {
                draw_text_ex(
                    &format!("Score: {}", score), 30.0, 30.0,
                    TextParams { font, color: BLACK, ..Default::default()}
                );
        
                draw_text_ex( 
                    &format!("Lifes: {} ", lives), screen_width() * 0.8, 30.0,
                    TextParams { font, color: BLACK, ..Default::default()}
                );
            }

            GameState::Dead => {
                clear_background(BROWN); 
                draw_text_ex(
                    &format!("Press SPACE to new game !"), screen_width() * 0.5, screen_height() * 0.5,
                    TextParams { font, color: BLACK, ..Default::default()}
                ); 
            }
        }

        next_frame().await
    }
}     
