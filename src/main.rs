#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
    stats: stats,
    swordAttack: swordAttack
}

impl character {
    fn new(x: f32, y: f32, direction: String, characterUp: [FRect; 3], characterDown: [FRect; 3], characterRight: [FRect; 3], characterLeft: [FRect; 3], health: f32) -> character {character{x, y, direction, walkingPhase: 0, characterUp, characterDown,characterRight, characterLeft, stats: stats::new(health), swordAttack: swordAttack::new("up".to_string())}}
    fn walkPhaseChange(&mut self, lastPhaseChange: &Instant) -> bool {
        if lastPhaseChange.elapsed().as_millis() >= 200 {
            self.walkingPhase += 1;
            if self.walkingPhase > 2 {
                self.walkingPhase = 1;
            }
            return true
        }

        false
    }
    fn render(&self, texture: &Texture, renderer: &mut WindowCanvas) {
        let rectRender = FRect::new(self.x, self.y, 70.0, 112.0);

        let rectSet = match self.direction.as_str() {
            "up" => self.characterUp,
            "down" => self.characterDown,
            "right" => self.characterRight,
            "left" => self.characterLeft,
            _ => self.characterDown
        };

        renderer.copy(texture, rectSet[self.walkingPhase], rectRender).unwrap();
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
        swingPhase: 6,
        swingPhaseFloat: 6.0,
        direction,
        swingCooldown: Instant::now()}}
    fn render (&mut self, renderer: &mut WindowCanvas, characterX: f32, characterY: f32, texture: &Texture) {
        if self.swingPhase == 6 { return; }
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

        match self.direction.as_str() {
            "up" => renderer.copy(texture, self.swingUp[self.swingPhase], FRect { x: renderX, y: renderY, w: self.swingUp[self.swingPhase].w * 7.0, h: self.swingUp[self.swingPhase].h * 7.0 }).unwrap(),
            "down" => renderer.copy(texture, self.swingDown[self.swingPhase], FRect { x: renderX, y: renderY, w: self.swingDown[self.swingPhase].w * 7.0, h: self.swingDown[self.swingPhase].h * 7.0 }).unwrap(),
            "left" => renderer.copy(texture, self.swingLeft[self.swingPhase], FRect { x: renderX, y: renderY, w: self.swingLeft[self.swingPhase].w * 7.0, h: self.swingLeft[self.swingPhase].h * 7.0 }).unwrap(),
            "right" => renderer.copy(texture, self.swingRight[self.swingPhase], FRect { x: renderX, y: renderY, w: self.swingRight[self.swingPhase].w * 7.0, h: self.swingRight[self.swingPhase].h * 7.0 }).unwrap(),
            _ => return
        }

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
    let videoSubsystem = sdl3::init().unwrap().video().unwrap();
    let window = videoSubsystem.window("Game", 1920, 1080).position_centered().resizable().opengl().build().unwrap();
    let mut renderer = create_renderer(window.clone(), None).unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    let rectUp = [FRect::new(2.0, 33.0, 9.0, 16.0), FRect::new(12.0, 33.0, 9.0, 16.0), FRect::new(22.0, 33.0, 9.0, 16.0)];
    let rectDown = [FRect::new(2.0, 49.0, 9.0, 16.0), FRect::new(12.0, 49.0, 9.0, 16.0), FRect::new(22.0, 49.0, 9.0, 16.0)];
    let rectRight = [FRect::new(1.0, 17.0, 10.0, 16.0), FRect::new(12.0, 17.0, 10.0, 16.0), FRect::new(23.0, 17.0, 10.0, 16.0)];
    let rectLeft = [FRect::new(1.0, 1.0, 10.0, 16.0), FRect::new(12.0, 1.0, 10.0, 16.0), FRect::new(23.0, 1.0, 10.0, 16.0)];
    let mainCharacter = character::new(window.size_in_pixels().0 as f32 / 2.0, window.size_in_pixels().1 as f32 / 2.0, "down".parse::<String>().unwrap(), rectUp, rectDown, rectRight, rectLeft, 20.0);

    gameLoop(&mut renderer, mainCharacter, window);
}

fn gameLoop(renderer: &mut WindowCanvas, mut mainCharacter: character, window: Window) {
    let mut lastPhaseChange = Instant::now();
    let surfaceMain = Surface::load_png("/home/rfitz/Backup/RustroverProjects/GameTest/src/Assets/gameCharacter.png").unwrap();
    let mut textureMain = renderer.create_texture_from_surface(&surfaceMain).unwrap();
    textureMain.set_scale_mode(sdl3::render::ScaleMode::Nearest);

    let surfaceSword = Surface::load_png("/home/rfitz/Backup/RustroverProjects/GameTest/src/Assets/mainSword.png").unwrap();
    let mut textureSword = renderer.create_texture_from_surface(&surfaceSword).unwrap();
    textureSword.set_scale_mode(sdl3::render::ScaleMode::Nearest);

    let surfaceCrest = Surface::load_png("/home/rfitz/Backup/RustroverProjects/GameTest/src/Assets/crest.png").unwrap();
    let textureCrest = renderer.create_texture_from_surface(&surfaceCrest).unwrap();

    let surfaceBackground = Surface::load_png("/home/rfitz/Backup/RustroverProjects/GameTest/src/Assets/grass.png").unwrap();
    let mut textureBackground = renderer.create_texture_from_surface(&surfaceBackground).unwrap();
    textureBackground.set_scale_mode(sdl3::render::ScaleMode::Nearest);

    let mut meshBox = meshBox::new(0.0, 0.0, 0.0, 0.0);
    let mut running = true;
    while running {
        let now = Instant::now();
        lastPhaseChange = input(&mut mainCharacter, lastPhaseChange, &mut running, &mut meshBox, &textureBackground, &window);
        //update(&mainCharacter);
        render(renderer, &mut mainCharacter, &textureMain, &textureBackground, &mut meshBox, &window, &textureCrest, &textureSword);
        let elapsed = now.elapsed().as_millis();
        let extraTime = 16 - elapsed;
        delay(extraTime as u32);
    }
}

fn input(mainCharacter: &mut character, mut lastPhaseChange: Instant, running: &mut bool, meshBox: &mut meshBox, backgroundTexture: &Texture, window: &Window) -> Instant {
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
        let phaseChanged = mainCharacter.walkPhaseChange(&lastPhaseChange);
        if phaseChanged {
            lastPhaseChange = Instant::now()
        }
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
    lastPhaseChange
}

//fn update(mainCharacter: &character) {

//}

fn render(renderer: &mut WindowCanvas, mainCharacter: &mut character, textureMain: &Texture, textureBackground: &Texture, meshBox: &mut meshBox, window: &Window, textureCrest: &Texture, swordTexture: &Texture) {
    renderer.set_draw_color(Color::RGB(50, 155, 50));
    renderer.clear();

    let backgroundRenderArea = FRect::new(0.0, 0.0, window.size_in_pixels().0 as f32, window.size_in_pixels().1 as f32);

    meshBox.render(textureBackground, renderer, backgroundRenderArea);
    mainCharacter.render(textureMain, renderer);
    mainCharacter.stats.render(renderer);
    mainCharacter.swordAttack.render(renderer, mainCharacter.x, mainCharacter.y, swordTexture);

    let crestRenderArea = FRect::new(0.0, 0.0, 104.0, 115.0);
    renderer.copy(textureCrest, None, crestRenderArea).unwrap();

    renderer.present();

}