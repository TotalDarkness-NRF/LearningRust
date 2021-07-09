use crate::{position::{Direction, Position}, terminal::Terminal};

pub struct Weapon {
    ammo: u16,
    clip: u8,
    clipSize: u16,
    fallOff: bool,
    weaponType: WeaponType,
    bulletsShot: Vec<Bullet>,
}

impl Weapon {
    pub fn createPistol() -> Self {
        Weapon {
            ammo: 50,
            clip: 5,
            clipSize: 50,
            fallOff: false,
            weaponType: WeaponType::Pistol(Bullet::new(2, '-')),
            bulletsShot: Vec::new(),
        }
    }

    pub fn createShotgun() -> Self {
        Weapon {
            ammo: 5,
            clip: 10,
            clipSize: 5,
            fallOff: true,
            weaponType: WeaponType::Shotgun(Bullet::new(10, '=')),
            bulletsShot: Vec::new(),
        }
    }

    pub fn createBow() -> Self {
        Weapon {
            ammo: 20,
            clip: 1,
            clipSize: 20,
            fallOff: false,
            weaponType: WeaponType::Bow(Bullet::new(5, '>')),
            bulletsShot: Vec::new(),
        }
    }

    pub fn shoot(&mut self, terminal: &mut Terminal, direction: Direction, position: Position) {
        if self.ammo == 0 {
            self.reload();
        } else if self.clip > 0 {
            self.ammo -= 1;
            let bullet: Bullet = self.weaponType.getBullet();
            self.bulletsShot.push(bullet);
            let bullet: &mut Bullet = self.bulletsShot.first_mut().unwrap();
            bullet.position = position;
            bullet.moves(terminal, direction);
        }
    }

    pub fn reload(&mut self) { //Dont check for ammo, if u reload u throw the clip away
        if self.clip > 0 {
            self.clip -= 1;
            self.ammo = self.clipSize;
        }
    }

    pub fn updateBullets(&mut self, terminal: &mut Terminal) {
        // TODO bullets dont get all updated. Only the most recent does
        let mut index: usize = 0;
        let mut toRemove: Vec<usize> = Vec::new();
        for bullet in self.bulletsShot.iter_mut() {
            if !bullet.update(terminal) {
                toRemove.push(index);
            }
            index += 1;
        }
        for index in toRemove {
            let bullet = self.bulletsShot.remove(index);
            terminal.eraseBox(bullet.getPosition());
        }
    }
}

enum WeaponType {
    Pistol(Bullet),
    Shotgun(Bullet),
    Bow(Bullet),
}

impl WeaponType {
    pub fn getBullet(&self) -> Bullet {
        match self {
            WeaponType::Pistol(bullet) => bullet.copy(),
            WeaponType::Shotgun(bullet) => bullet.copy(),
            WeaponType::Bow(arrow) => arrow.copy(),
        }
    }
}

pub struct Bullet {
    direction: Direction,
    position: Position,
    timeAlive: u16,
    damage: u8,
    icon: char,
}

impl Bullet {
    pub fn new(damage: u8, icon: char) -> Self {
        Bullet {direction: Direction::None, position: Position::new(0, 0), timeAlive: 0, damage, icon}
    }

    fn copy(&self) -> Self {
        Bullet::new(self.damage, self.icon)
    }

    pub fn increaseTime(&mut self) {
        self.timeAlive += 1;
    }
    
    pub fn moves(&mut self, terminal: &mut Terminal, direction: Direction) -> bool{
        terminal.eraseBox(&self.position);
        self.direction = direction;
        let hasMoved = match self.direction {
            Direction::Up => self.position.moveUp(),
            Direction::Down => self.position.moveDown(),
            Direction::Left => self.position.moveLeft(),
            Direction::Right => self.position.moveRight(),
            Direction::None => false,
        };
        terminal.drawChar(&self.position, self.icon);
        hasMoved
    }

    pub fn update(&mut self, terminal: &mut Terminal) -> bool {
        self.increaseTime();
        match self.direction {
            Direction::None => {self.timeAlive = 0; return false;},
            Direction::Up => self.moves(terminal, Direction::Up),
            Direction::Down => self.moves(terminal, Direction::Down),
            Direction::Left => self.moves(terminal, Direction::Left),
            Direction::Right => self.moves(terminal, Direction::Right),
        }
    }

    pub fn getDirection(&self) -> &Direction {
        &self.direction
    }

    pub fn getPosition(&self) -> &Position {
        &self.position
    }

    pub fn getTimeAlive(&self) -> u16 {
        self.timeAlive
    }

    pub fn getDamage(&self) -> u8 {
        self.damage
    }

    pub fn getIcon(&self) -> char {
        self.icon
    }
}