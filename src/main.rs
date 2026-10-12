#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use sdl3::keyboard::{Scancode};
use sdl3::pixels::{Color};
use sdl3::render::{create_renderer, Texture, WindowCanvas, FRect};
use sdl3::surface::Surface;
use std::time::{Instant};
use sdl3::timer::delay;
use sdl3::video::Window;

struct character {
    x: f32,
    y: f32,
    direction: String,
    walkingPhase: usize,
    characterUp: [FRect; 3],
    characterDown: [FRect; 3],
    characterRight: [FRect; 3],
    characterLeft: [FRect; 3],
    hitBox: FRect,
    stats: stats,
    swordAttack: swordAttack,
    lastPhaseChange: Instant,
    immune: bool,
    render: bool,
    timeSinceHit: Instant,
}

impl character {
    fn new(x: f32, y: f32, direction: String, health: f32) -> character {character{
        x,
        y,
        direction,
        walkingPhase: 0,
        characterUp: [FRect::new(2.0, 33.0, 9.0, 16.0), FRect::new(12.0, 33.0, 9.0, 16.0), FRect::new(22.0, 33.0, 9.0, 16.0)],
        characterDown: [FRect::new(2.0, 49.0, 9.0, 16.0), FRect::new(12.0, 49.0, 9.0, 16.0), FRect::new(22.0, 49.0, 9.0, 16.0)] ,
        characterRight: [FRect::new(1.0, 17.0, 10.0, 16.0), FRect::new(12.0, 17.0, 10.0, 16.0), FRect::new(23.0, 17.0, 10.0, 16.0)],
        characterLeft: [FRect::new(1.0, 1.0, 10.0, 16.0), FRect::new(12.0, 1.0, 10.0, 16.0), FRect::new(23.0, 1.0, 10.0, 16.0)],
        hitBox: FRect{x, y, w: 70.0, h: 112.0},
        stats: stats::new(health),
        swordAttack: swordAttack::new("up".to_string()),
        lastPhaseChange: Instant::now(),
        immune: false,
        render: true,
        timeSinceHit: Instant::now(),
    }}
    fn walkPhaseChange(&mut self) {
        if self.lastPhaseChange.elapsed().as_millis() >= 200 {
            self.walkingPhase += 1;
            if self.walkingPhase > 2 {
                self.walkingPhase = 1;
            }
            self.lastPhaseChange = Instant::now();
        }
    }
    fn update (&mut self) {
        if self.timeSinceHit.elapsed().as_millis() <= 250 {
            self.render = !self.render;
            self.immune = true;
        }
        else {
            self.immune = false;
            self.render = true;
        }
    }
    fn render(&mut self, texture: &Texture, renderer: &mut WindowCanvas) {
        self.hitBox.x = self.x;
        self.hitBox.y = self.y;

        let rectSet = match self.direction.as_str() {
            "up" => self.characterUp,
            "down" => self.characterDown,
            "right" => self.characterRight,
            "left" => self.characterLeft,
            _ => self.characterDown
        };

        self.hitBox.w = rectSet[self.walkingPhase].w * 7.0;
        self.hitBox.h = rectSet[self.walkingPhase].h * 7.0;

        renderer.copy(texture, rectSet[self.walkingPhase], self.hitBox).unwrap();
    }
    fn updatePosition(&mut self, x: f32, y: f32, window: &Window) {
        self.x += x;
        self.y += y;

        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.x + 70.0 > 0.0 + window.size_in_pixels().0 as f32 {
            self.x = window.size_in_pixels().0 as f32 - 70.0;
        }
        if self.y + 105.0 > 0.0 + window.size_in_pixels().1 as f32 {
            self.y = window.size_in_pixels().1 as f32 - 105.0;
        }
    }
}

struct wolf {
    x: f32,
    y: f32,
    direction: String,
    walkingPhase: usize,
    wolfUp: [FRect; 3],
    wolfDown: [FRect; 3],
    wolfRight: [FRect; 3],
    wolfLeft: [FRect; 3],
    hitBox: FRect,
    health: f32,
    playerFound: bool,
    lastPhaseChange: Instant,
    render: bool,
    timeSinceHit: Instant,
}

impl wolf {
    fn new(x: f32, y: f32, direction: String) -> wolf {wolf{
        x,
        y,
        direction,
        walkingPhase: 0,
        wolfUp: [FRect::new(2.0, 29.0, 14.0, 27.0), FRect::new(17.0, 29.0, 14.0, 27.0), FRect::new(32.0, 29.0, 14.0, 27.0)],
        wolfDown: [FRect::new(1.0, 1.0, 15.0, 27.0), FRect::new(18.0, 1.0, 15.0, 27.0), FRect::new(35.0, 1.0, 15.0, 27.0)],
        wolfLeft: [FRect::new(1.0, 57.0, 33.0, 28.0), FRect::new(35.0, 57.0, 33.0, 28.0), FRect::new(69.0, 57.0, 33.0, 28.0)],
        wolfRight: [FRect::new(1.0, 86.0, 33.0, 28.0), FRect::new(35.0, 86.0, 33.0, 28.0), FRect::new(70.0, 86.0, 33.0, 28.0)],
        hitBox: FRect{x, y, w: 0.0, h: 0.0},
        health: 4.0,
        playerFound: true,
        lastPhaseChange: Instant::now(),
        render: true,
        timeSinceHit: Instant::now(),
    }}
    fn render(&mut self, texture: &Texture,renderer: &mut WindowCanvas, meshBox: &mut meshBox) {
        self.hitBox.x = (self.x - meshBox.x) * 6.0;
        self.hitBox.y = (self.y - meshBox.y) * 6.0;

        let rectSet = match self.direction.as_str() {
            "up" => self.wolfUp,
            "down" => self.wolfDown,
            "right" => self.wolfRight,
            "left" => self.wolfLeft,
            _ => self.wolfDown
        };

        self.hitBox.w = rectSet[self.walkingPhase].w * 6.0;
        self.hitBox.h = rectSet[self.walkingPhase].h * 6.0;

        self.hitBox.x -= self.hitBox.w / 2.0;
        self.hitBox.y -= self.hitBox.h / 2.0;

        renderer.copy(texture, rectSet[self.walkingPhase], self.hitBox).unwrap();
    }
    fn findPlayer (&mut self, mainCharacter: &character, backgroundTexture: &Texture, meshBox: &meshBox) {
        //if !self.playerFound{
        //    if
        //}
        if self.lastPhaseChange.elapsed().as_millis() >= 200 {
            self.walkingPhase += 1;
            if self.walkingPhase > 2 {
                self.walkingPhase = 1;
            }
            self.lastPhaseChange = Instant::now();
        }
        if self.timeSinceHit.elapsed().as_millis() <= 250 {
            self.render = !self.render;
        }
        else {
            self.render = true;
        }

        if self.playerFound {
            let speed = 0.7;
            let mainPosX = mainCharacter.x + meshBox.x * 6.0;
            let mainPosY = mainCharacter.y + meshBox.y * 6.0;

            let dx = mainPosX - self.x * 6.0;
            let dy = mainPosY - self.y * 6.0;

            let mut vertical = false;

            let mut dxRatio = dx / dy;
            if dxRatio < 0.0{
                dxRatio = -dxRatio;
            }
            dxRatio += 1.0;

            let mut my = speed / dxRatio;
            let mut mx = speed - my;
            if my > speed {
                mx = speed / dxRatio;
                my = speed - my;
            }
            if mx < my {
                vertical = true;
            }

            if dx < 0.0 {
                mx = -mx;
            }
            if dy < 0.0 {
                my = -my;
            }
            if vertical == true {
                if my > 0.0 {
                    self.direction = String::from("down");
                }
                else {
                    self.direction = String::from("up");
                }
            }
            else {
                if mx > 0.0 {
                    self.direction = String::from("right");
                }
                else {
                    self.direction = String::from("left");
                }
            }

            self.updatePosition(mx, my, backgroundTexture);
        }
    }
    fn updatePosition(&mut self, x: f32, y: f32, backgroundTexture: &Texture) {
        self.x += x;
        self.y += y;

        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.x + self.hitBox.w > backgroundTexture.width() as f32 * 6.0 {
            self.x = backgroundTexture.width() as f32 - self.hitBox.w;
        }
        if self.y + self.hitBox.h > backgroundTexture.height() as f32 * 6.0 {
            self.y = backgroundTexture.height() as f32 - self.hitBox.h;
        }
    }
}

struct stats {
    health: f32,
    maxHealth: f32,
    stamina: f32,
    maxStamina: f32,
    mana: f32,
    maxMana: f32,
}

impl stats {
    fn new(health: f32) -> stats {stats{health, maxHealth: health, stamina:10.0, maxStamina:10.0, mana:10.0, maxMana:10.0}}
    fn render(&self, renderer: &mut WindowCanvas) {
        let maxHealthRenderBox = FRect::new(
            105.0,
            10.0,
            self.maxHealth * 25.0,
            25.0,
        );
        let healthRenderBox = FRect::new(
            108.0,
            13.0,
            self.health * 25.0 - 6.0,
            19.0,
        );
        renderer.set_draw_color(Color::RGB(180, 180, 180));
        renderer.fill_rect(maxHealthRenderBox).unwrap();
        renderer.set_draw_color(Color::RGB(130, 20, 20));
        renderer.fill_rect(healthRenderBox).unwrap();

        let maxStaminaRenderBox = FRect::new(
            105.0,
            47.0,
            self.maxStamina * 50.0,
            25.0,
        );
        let staminaRenderBox = FRect::new(
            108.0,
            50.0,
            self.stamina * 50.0 - 8.0,
            19.0,
        );
        renderer.set_draw_color(Color::RGB(180, 180, 180));
        renderer.fill_rect(maxStaminaRenderBox).unwrap();
        renderer.set_draw_color(Color::RGB(30, 130, 30));
        renderer.fill_rect(staminaRenderBox).unwrap();

        let maxManaRenderBox = FRect::new(
            105.0,
            85.0,
            self.maxMana * 50.0,
            25.0,
        );
        let manaRenderBox = FRect::new(
            108.0,
            88.0,
            self.mana * 50.0 - 6.0,
            19.0,
        );
        renderer.set_draw_color(Color::RGB(180, 180, 180));
        renderer.fill_rect(maxManaRenderBox).unwrap();
        renderer.set_draw_color(Color::RGB(80, 0, 140));
        renderer.fill_rect(manaRenderBox).unwrap();
    }
}

struct swordAttack {
    swingUp: [FRect; 6],
    swingDown: [FRect; 6],
    swingLeft: [FRect; 6],
    swingRight: [FRect; 6],
    hitBox: FRect,
    swingPhase: usize,
    swingPhaseFloat: f32,
    direction: String,
    swingCooldown: Instant,
}

impl swordAttack{
    fn new(direction: String) -> swordAttack {swordAttack{
        swingUp: [FRect::new(1.0, 1.0, 3.0, 3.0), FRect::new(5.0, 1.0, 13.0, 11.0), FRect::new(19.0, 1.0, 25.0, 13.0), FRect::new(47.0, 1.0, 31.0, 11.0), FRect::new(79.0, 1.0, 33.0, 10.0), FRect::new(113.0, 1.0, 32.0, 14.0)],
        swingDown: [FRect::new(1.0, 16.0, 3.0, 3.0), FRect::new(5.0, 16.0, 13.0, 11.0), FRect::new(19.0, 16.0, 25.0, 13.0), FRect::new(47.0, 16.0, 31.0, 11.0), FRect::new(79.0, 16.0, 33.0, 11.0), FRect::new(113.0, 16.0, 32.0, 14.0)],
        swingLeft: [FRect::new(1.0, 30.0, 3.0, 2.0), FRect::new(5.0, 30.0, 12.0, 9.0), FRect::new(17.0, 30.0, 12.0, 15.0), FRect::new(30.0, 30.0, 11.0, 20.0), FRect::new(42.0, 30.0, 10.0, 21.0), FRect::new(53.0, 30.0, 14.0, 20.0)],
        swingRight: [FRect::new(1.0, 53.0, 3.0, 2.0), FRect::new(5.0, 53.0, 12.0, 8.0), FRect::new(17.0, 53.0, 12.0, 15.0), FRect::new(30.0, 53.0, 11.0, 20.0), FRect::new(42.0, 53.0, 10.0, 21.0), FRect::new(53.0, 53.0, 14.0, 20.0)],
        hitBox: FRect{x: 0.0, y: 0.0, w: 0.0, h: 0.0},
        swingPhase: 6,
        swingPhaseFloat: 6.0,
        direction,
        swingCooldown: Instant::now()}}
    fn render (&mut self, renderer: &mut WindowCanvas, characterX: f32, characterY: f32, texture: &Texture) {
        if self.swingPhase == 6 {
            self.hitBox.w = 0.0;
            self.hitBox.h = 0.0;
            self.hitBox.x = 0.0;
            self.hitBox.y = 0.0;
            return;
        }
        let renderX:f32 = match self.direction.as_str() {
            "up" => match self.swingPhase as f32 {
                0.0 | 1.0 | 2.0 => characterX - 19.5 * 7.0 + 35.0,
                3.0 => characterX - 17.5 * 7.0 + 35.0,
                4.0 => characterX - 14.5 * 7.0 + 35.0,
                5.0 => characterX - 11.5 * 7.0 + 35.0,
                _ => 0.0,
            }
            "down" => match self.swingPhase as f32 {
                0.0 => characterX + 16.5 * 7.0 + 35.0,
                1.0 => characterX + 6.5 * 7.0 + 35.0,
                2.0 => characterX + -5.5 * 7.0 + 35.0,
                3.0 => characterX + -11.5 * 7.0 + 35.0,
                4.0 => characterX + -16.5 * 7.0 + 35.0,
                5.0 => characterX + -18.5 * 7.0 + 35.0,
                _ => 0.0,
            },
            "left" => match self.swingPhase as f32 {
                0.0 => characterX - 3.0 * 7.9 - 30.0,
                1.0  => characterX - 11.0 * 7.0 - 30.0,
                2.0 |3.0 |4.0 | 5.0 => characterX - 12.0 * 7.0 - 30.0,

                _ => 0.0,
            },
            "right" => match self.swingPhase as f32 {
                0.0 | 1.0 | 2.0  => characterX + 90.0,
                3.0 => characterX + 1.0 * 7.0 + 100.0,
                4.0 => characterX + 2.0 * 7.0 + 100.0,
                5.0 => characterX - 2.0 * 7.0 + 100.0,
                _ => 0.0,
            },
            _ => 0.0,
        };
        let renderY:f32 = match self.direction.as_str() {
            "up" => match self.swingPhase as f32 {
                0.0 => characterY,
                1.0 => characterY - 8.0 * 7.0,
                2.0 | 3.0 | 4.0 | 5.0 => characterY - 10.0 * 7.0,
                _ => 0.0
            }
            "down" => match self.swingPhase as f32 {
                0.0 | 1.0 | 2.0 | 5.0 => characterY + 95.0,
                3.0 |4.0 => characterY + 109.0,
                _ => 0.0
            }
            "left" => match self.swingPhase as f32 {
                0.0 => characterY - 3.0 * 7.0 + 140.0,
                1.0 => characterY - 10.0 * 7.0 + 140.0,
                2.0 => characterY - 16.0 * 7.0 + 140.0,
                3.0 => characterY - 22.0 * 7.0 + 140.0,
                4.0 => characterY - 25.0 * 7.0 + 140.0,
                5.0 => characterY - 26.0 * 7.0 + 140.0,
                _ => 0.0
            }
            "right" => match self.swingPhase as f32 {
                0.0 | 1.0 | 2.0 | 3.0 => characterY - 35.0,
                4.0 => characterY + 3.0 * 7.0 - 35.0,
                5.0 => characterY + 5.0 * 7.0 - 35.0,
                _ => 0.0,
            }
            _ => 0.0,
        };

        self.hitBox = match self.direction.as_str() {
            "up" => FRect { x: renderX, y: renderY, w: self.swingUp[self.swingPhase].w * 7.0, h: self.swingUp[self.swingPhase].h * 7.0 },
            "down" => FRect { x: renderX, y: renderY, w: self.swingDown[self.swingPhase].w * 7.0, h: self.swingDown[self.swingPhase].h * 7.0 },
            "left" => FRect { x: renderX, y: renderY, w: self.swingLeft[self.swingPhase].w * 7.0, h: self.swingLeft[self.swingPhase].h * 7.0 },
            "right" => FRect { x: renderX, y: renderY, w: self.swingRight[self.swingPhase].w * 7.0, h: self.swingRight[self.swingPhase].h * 7.0 },
            _ => FRect::new(0.0, 0.0, 0.0, 0.0),
        };

        match self.direction.as_str() {
            "up" => renderer.copy(texture, self.swingUp[self.swingPhase], self.hitBox).unwrap(),
            "down" => renderer.copy(texture, self.swingDown[self.swingPhase], self.hitBox).unwrap(),
            "left" => renderer.copy(texture, self.swingLeft[self.swingPhase], self.hitBox).unwrap(),
            "right" => renderer.copy(texture, self.swingRight[self.swingPhase], self.hitBox).unwrap(),
            _ => return
        };

        self.swingPhaseFloat += 0.5;
        self.swingPhase = self.swingPhaseFloat.floor() as usize;
    }
}

struct meshBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    endeOfDirection: Vec<String>
}

impl meshBox {
    fn new(x: f32, y: f32, width: f32, height: f32) -> meshBox {meshBox{x, y, width, height, endeOfDirection: Vec::new()}}
    fn render(&mut self, texture: &Texture, renderer: &mut WindowCanvas, renderArea: FRect) {
        let textureArea = FRect::new(self.x, self.y, renderArea.w / 6.0, renderArea.h / 6.0 );

        self.width = renderArea.w /6.0;
        self.height = renderArea.h /6.0;

        renderer.copy(texture, textureArea, renderArea).unwrap();
    }
    fn updatePosition(&mut self, mx: f32, my: f32, texture: &Texture) {
        self.x += mx;
        self.y += my;
        self.endeOfDirection.clear();

        if self.x <= 0.0{
            self.x = 0.0;
            self.endeOfDirection.push("left".to_string());
        }

        if self.x + self.width >= texture.width() as f32{
            self.x = texture.width() as f32 - self.width;
            self.endeOfDirection.push("right".to_string());
        }

        if self.y <= 0.0{
            self.y = 0.0;
            self.endeOfDirection.push("up".to_string());
        }

        if self.y + self.height >= texture.height() as f32{
            self.y = texture.height() as f32 - self.height;
            self.endeOfDirection.push("down".to_string());
        }
    }
}

fn main() {
    let dir = std::env::current_dir().unwrap();
    println!("Working directory: {}", dir.display()); //If the code is not working, chances are it is not properly opening the files.
    let videoSubsystem = sdl3::init().unwrap().video().unwrap();
    let window = videoSubsystem.window("Game", 1920, 1080).position_centered().resizable().opengl().build().unwrap();
    let mut renderer = create_renderer(window.clone(), None).unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    gameLoop(&mut renderer, window);
}

fn gameLoop(renderer: &mut WindowCanvas, window: Window) {
    let mut textureVec: Vec<Texture> = Vec::new();
    let mut mainCharacter = character::new(window.size_in_pixels().0 as f32 / 2.0, window.size_in_pixels().1 as f32 / 2.0, "down".parse::<String>().unwrap(), 20.0);

    let mut wolfVec: Vec<wolf> = Vec::new();
    wolfVec.push(wolf::new(100.0, 100.0, "down".to_string()));
    let mut wolfTimer = Instant::now();
    let mut score = 0;

    let surfaceBackground = Surface::load_png("src/Assets/grass.png").expect("Failed to load assets/grass.png. Make sure you are running the project from the 'GameTest' folder rather than the 'src' folder. If for whatever reason you cannot do that, feel free to remove '/src' from all lines containing. These lines are also marked with a comment saying 'here'"); //here
    let mut textureBackground = renderer.create_texture_from_surface(&surfaceBackground).unwrap();
    textureBackground.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureBackground);

    let surfaceMain = Surface::load_png("src/Assets/gameCharacter.png").unwrap(); //here
    let mut textureMain = renderer.create_texture_from_surface(&surfaceMain).unwrap();
    textureMain.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureMain);

    let surfaceSword = Surface::load_png("src/Assets/mainSword.png").unwrap(); //here
    let mut textureSword = renderer.create_texture_from_surface(&surfaceSword).unwrap();
    textureSword.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureSword);

    let surfaceCrest = Surface::load_png("src/Assets/crest.png").unwrap(); //here
    let mut textureCrest = renderer.create_texture_from_surface(&surfaceCrest).unwrap();
    textureCrest.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureCrest);

    let surfaceWolf = Surface::load_png("src/Assets/wolf.png").unwrap(); //here
    let mut textureWolf = renderer.create_texture_from_surface(&surfaceWolf).unwrap();
    textureWolf.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    textureVec.push(textureWolf);

    let mut meshBox = meshBox::new(0.0, 0.0, 0.0, 0.0);
    let mut running = true;
    let mut alive = true;

    while alive && running {
        if wolfTimer.elapsed().as_secs() > 3 {
            wolfVec.push( wolf::new(rand::random_range(0.0 ..textureVec[0].width() as f32), rand::random_range(0.0..textureVec[0].height() as f32), "down".to_string()));
            wolfTimer = Instant::now();
        }
        let now = Instant::now();
        input(&mut mainCharacter, &mut running, &mut meshBox, &textureVec[0], &window);
        update(&mut mainCharacter, &mut wolfVec, &textureVec[0], &mut meshBox, &mut alive, &mut score);
        render(renderer, &mut mainCharacter, &mut meshBox, &window, &textureVec, &mut wolfVec);
        let mut elapsed = now.elapsed().as_millis();
        if elapsed > 16 {
            elapsed = 0;
        }
        let extraTime = 16 - elapsed;
        delay(extraTime as u32);
    }
    delay(250);

    let path = Path::new("src/Assets/DovesTypeText-Regular.ttf"); //here
    let ttfContext = sdl3::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttfContext.load_font(path, 100.0).expect("SDL_TTF failed to load font.");

    let gameOverSurface = font.render("You Died").blended(Color::RGB(150, 50, 50)).unwrap();
    let gameOverTexture = renderer.create_texture_from_surface(&gameOverSurface).unwrap();

    let scoreSurface = font.render(&*("Score: ".to_string() + &*score.to_string())).blended(Color::RGB(200, 200, 200)).unwrap();
    let scoreTexture = renderer.create_texture_from_surface(&scoreSurface).unwrap();

    let mut file = File::open("src/Assets/highscore.txt").unwrap(); //here
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let highScoreSurface = font.render(&*("High Score: ".to_string() + &buffer)).blended(Color::RGB(200, 200, 200)).unwrap();
    let highScoreTexture = renderer.create_texture_from_surface(&highScoreSurface).unwrap();


    while running {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();


        let deathMsgRect = FRect::new(
            window.size_in_pixels().0 as f32 / 4.0,
            50.0,
            window.size_in_pixels().0 as f32 / 2.0,
            window.size_in_pixels().1 as f32 / 3.0,
        );
        let mut highScoreMsgRect = FRect::new(
          deathMsgRect.x + deathMsgRect.w / 3.0,
          deathMsgRect.y + deathMsgRect.h + 50.0,
          deathMsgRect.w / 3.0,
          deathMsgRect.h / 3.0,
        );

        renderer.copy(&gameOverTexture, None, deathMsgRect).unwrap();
        renderer.copy(&scoreTexture, None, highScoreMsgRect).unwrap();
        highScoreMsgRect.y += highScoreMsgRect.h + 50.0;
        highScoreMsgRect.h /= 1.5;
        renderer.copy(&highScoreTexture, None, highScoreMsgRect).unwrap();



        renderer.present();
        input(&mut mainCharacter, &mut running, &mut meshBox, &textureVec[0], &window);
    }
    if score > buffer.parse::<i32>().unwrap() {
        let mut newFile = File::create("src/Assets/highscore.txt").unwrap(); //here
        newFile.write_all(&score.to_string().as_bytes()).unwrap();
    }
}

fn hitboxDetector(hitbox1: FRect, hitbox2: FRect) -> bool {
    let mut touching = true;

    let top1 = hitbox1.y;
    let bottom1 = hitbox1.y + hitbox1.h;
    let left1 = hitbox1.x;
    let right1 = hitbox1.x + hitbox1.w;

    let top2 = hitbox2.y;
    let bottom2 = hitbox2.y + hitbox2.h;
    let left2 = hitbox2.x;
    let right2 = hitbox2.x + hitbox2.w;

    if bottom1 < top2{
        touching = false;
    }
    if top1 > bottom2{
        touching = false;
    }
    if left1 > right2{
        touching = false;
    }
    if right1 < left2{
        touching = false;
    }

    touching
}

fn input(mainCharacter: &mut character, running: &mut bool, meshBox: &mut meshBox, backgroundTexture: &Texture, window: &Window) {
    let mut eventPump = sdl3::init().unwrap().event_pump().unwrap();
    let keyboardState = eventPump.keyboard_state();
    let mut speed = 1.0;
    let mut walking = false;
    let mut arrowKeys = false;
    let mut moveBackInDirection = Vec::new();
    let mut moveMainInDirection = Vec::new();
    let mut usedStamina = false;

    if (keyboardState.is_scancode_pressed(Scancode::LShift) || keyboardState.is_scancode_pressed(Scancode::RShift)) && mainCharacter.stats.stamina > 0.0 {
        speed *= 1.5;
        mainCharacter.stats.stamina -= 0.05;
        usedStamina = true;
    }

    if keyboardState.is_scancode_pressed(Scancode::Up){
        mainCharacter.direction = "up".to_string();
        arrowKeys = true;
    }
    if keyboardState.is_scancode_pressed(Scancode::Down){
        mainCharacter.direction = "down".to_string();
        arrowKeys = true;
    }
    if keyboardState.is_scancode_pressed(Scancode::Left){
        mainCharacter.direction = "left".to_string();
        arrowKeys = true;
    }
    if keyboardState.is_scancode_pressed(Scancode::Right){
        mainCharacter.direction = "right".to_string();
        arrowKeys = true;
    }
    if mainCharacter.swordAttack.swingPhase != 6 {
        mainCharacter.direction = mainCharacter.swordAttack.direction.clone();
        arrowKeys = true;
    }

    if keyboardState.is_scancode_pressed(Scancode::W) && !keyboardState.is_scancode_pressed(Scancode::S)  {
        walking = true;
        if !arrowKeys {
            mainCharacter.direction = "up".to_string();
        }
        if meshBox.endeOfDirection.contains(&"up".to_string()) || mainCharacter.y > window.size_in_pixels().1 as f32 / 2.0 {
            moveMainInDirection.push("up".to_string());
        }
        else {
            moveBackInDirection.push("up".to_string());
        }
    }
    if keyboardState.is_scancode_pressed(Scancode::S) && !keyboardState.is_scancode_pressed(Scancode::W) {
        walking = true;
        if !arrowKeys {
            mainCharacter.direction = "down".to_string();
        }
        if meshBox.endeOfDirection.contains(&"down".to_string()) || mainCharacter.y < window.size_in_pixels().1 as f32 / 2.0{
            moveMainInDirection.push("down".to_string());
        }
        else {
            moveBackInDirection.push("down".to_string());
        }
    }
    if keyboardState.is_scancode_pressed(Scancode::A) && !keyboardState.is_scancode_pressed(Scancode::D) {
        walking = true;
        if !arrowKeys {
            mainCharacter.direction = "left".to_string();
        }
        if meshBox.endeOfDirection.contains(&"left".to_string()) || mainCharacter.x > window.size_in_pixels().0 as f32 / 2.0 {
            moveMainInDirection.push("left".to_string());
        }
        else {
            moveBackInDirection.push("left".to_string());
        }
    }
    if keyboardState.is_scancode_pressed(Scancode::D) && !keyboardState.is_scancode_pressed(Scancode::A) {
        walking = true;
        if !arrowKeys {
            mainCharacter.direction = "right".to_string();
        }
        if meshBox.endeOfDirection.contains(&"right".to_string()) || mainCharacter.x < window.size_in_pixels().0 as f32 / 2.0 {
            moveMainInDirection.push("right".to_string());
        }
        else {
            moveBackInDirection.push("right".to_string());
        }
    }

    if keyboardState.is_scancode_pressed(Scancode::X) && mainCharacter.swordAttack.swingCooldown.elapsed().as_millis() > 350 && mainCharacter.stats.stamina > 1.0 {
        mainCharacter.swordAttack.swingPhase = 0;
        mainCharacter.swordAttack.swingPhaseFloat = 0.0;
        mainCharacter.swordAttack.direction = mainCharacter.direction.clone();
        mainCharacter.stats.stamina -= 3.0;
        mainCharacter.swordAttack.swingCooldown = Instant::now();
        usedStamina = true;
    }

    let directions = [moveBackInDirection.clone(), moveMainInDirection.clone()].concat();
    if !directions.contains(&mainCharacter.direction){
        speed /= 3.0;
    }
    if (directions.contains(&"up".to_string()) || directions.contains(&"down".to_string())) && (directions.contains(&"left".to_string())  || directions.contains(&"right".to_string())) {
        speed /= 1.5;
    }

    for direction in moveMainInDirection {
        match direction.as_str() {
            "up" => mainCharacter.updatePosition(0.0, -5.0 * speed, window),
            "down" => mainCharacter.updatePosition(0.0, 5.0 * speed, window),
            "left" => mainCharacter.updatePosition(-5.0 * speed, 0.0, window),
            "right" => mainCharacter.updatePosition(5.0 * speed, 0.0, window),
            &_ => {}
        }
    }
    for direction in moveBackInDirection {
        match direction.as_str() {
            "up" => meshBox.updatePosition(0.0, -1.0 * speed, backgroundTexture),
            "down" => meshBox.updatePosition(0.0, 1.0 * speed, backgroundTexture),
            "left" => meshBox.updatePosition(-1.0 * speed, 0.0, backgroundTexture),
            "right" => meshBox.updatePosition(1.0 * speed, 0.0, backgroundTexture),
            &_ => {}
        }
    }

    if walking {
        mainCharacter.walkPhaseChange();

        if !usedStamina &&mainCharacter.stats.stamina < mainCharacter.stats.maxStamina {
            mainCharacter.stats.stamina += 0.04;
        }
    }
    else {
        mainCharacter.walkingPhase = 0;
        if mainCharacter.stats.stamina < mainCharacter.stats.maxStamina - 0.1 {
            mainCharacter.stats.stamina += 0.05;
        }
    }
    for event in eventPump.poll_iter() {
        use sdl3::event::Event;
        match event {
            Event::Quit { .. } =>  {
                *running = false;
            }
            _ => {}
        }
    }
}

fn update(mainCharacter: &mut character, wolfVec: &mut Vec<wolf>, backgroundTexture: &Texture, meshBox: &mut meshBox, alive: &mut bool, score: &mut i32) {
    mainCharacter.update();
    for wolf in wolfVec.iter_mut() {
        wolf.findPlayer(mainCharacter, backgroundTexture, meshBox);
        if hitboxDetector(wolf.hitBox, mainCharacter.hitBox) && !mainCharacter.immune {
            mainCharacter.timeSinceHit = Instant::now();
            mainCharacter.stats.health -= 2.0;
            if mainCharacter.stats.health <= 0.0 {
                *alive = false;
            }
        }
        if hitboxDetector(wolf.hitBox, mainCharacter.swordAttack.hitBox) {
            let speed = 2.0;
            let mainPosX = mainCharacter.x + meshBox.x * 6.0;
            let mainPosY = mainCharacter.y + meshBox.y * 6.0;

            let dx = mainPosX - wolf.x * 6.0;
            let dy = mainPosY - wolf.y * 6.0;

            let mut dxRatio = dx / dy;
            if dxRatio < 0.0{
                dxRatio = -dxRatio;
            }
            dxRatio += 1.0;

            let mut my = speed / dxRatio;
            let mut mx = speed - my;
            if my > speed {
                mx = speed / dxRatio;
                my = speed - my;
            }

            if dx < 0.0 {
                mx = -mx;
            }
            if dy < 0.0 {
                my = -my;
            }

            wolf.health -= 0.1;
            wolf.updatePosition(-mx, -my, backgroundTexture);
            wolf.timeSinceHit = Instant::now();
        }
    }
    let mut removeWolf = Vec::new();
    for wolf in 0..wolfVec.len() {
        if wolfVec[wolf].health < 0.0 {
            removeWolf.push(wolf);
        }
    }
    removeWolf.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for wolf in removeWolf {
        wolfVec.remove(wolf);
        *score += 1;
    }
}

fn render(renderer: &mut WindowCanvas, mainCharacter: &mut character, meshBox: &mut meshBox, window: &Window, textureVec: &Vec<Texture>, wolfVec: &mut Vec<wolf>) {
    renderer.set_draw_color(Color::RGB(50, 155, 50));
    renderer.clear();

    let backgroundRenderArea = FRect::new(0.0, 0.0, window.size_in_pixels().0 as f32, window.size_in_pixels().1 as f32);
    let crestRenderArea = FRect::new(0.0, 0.0, 104.0, 115.0);

    meshBox.render(&textureVec[0], renderer, backgroundRenderArea);
    if mainCharacter.render {
        mainCharacter.render(&textureVec[1], renderer);
    }
    for wolf in wolfVec.iter_mut() {
        if wolf.render {
            wolf.render(&textureVec[4], renderer, meshBox);
        }
    }
    mainCharacter.swordAttack.render(renderer, mainCharacter.x, mainCharacter.y, &textureVec[2]);
    mainCharacter.stats.render(renderer);

    renderer.copy(&textureVec[3], None, crestRenderArea).unwrap();

    renderer.present();

}