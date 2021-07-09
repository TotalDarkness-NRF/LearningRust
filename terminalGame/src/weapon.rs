use crate::{position::{Direction, Position}, terminal::Terminal};

pub struct Weapon {
    ammo: u16,
    clip: u8,
    clipSize: u16,
    fallOff: bool,
    weaponType: WeaponType,
    bulletsShot: Vec<Bullet>,
}

impl ToString for Weapon {
    fn to_string(&self) -> String {
        format!("ammo:{} clip:{} falloff:{} type:{} bulletsShot:{}",
        self.ammo, self.clip, self.fallOff, self.weaponType.getType(), self.bulletsShot.len())
    }
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

    pub fn shoot(&mut self, direction: Direction, position: Position) {
        if self.ammo == 0 {
            self.reload();
        } else {
            self.ammo -= 1;
            let mut bullet: Bullet = self.weaponType.getBullet();
            bullet.setPosition(position);
            bullet.setDiection(direction);
            self.bulletsShot.push(bullet);
        }
    }

    pub fn reload(&mut self) { 
        //Dont check for ammo, if u reload u throw the clip away
        if self.clip > 0 {
            self.clip -= 1;
            self.ammo = self.clipSize;
        }
    }

    pub fn updateBullets(&mut self, terminal: &mut Terminal) {
        let mut index: usize = 0;
        let mut toRemove: Vec<usize> = Vec::new();
        for bullet in self.bulletsShot.iter_mut() {
            if !bullet.update(terminal) {
                toRemove.push(index);
            }
            index += 1;
        }
        toRemove.reverse(); // To avoid having changing index we remove from back first
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
    fn getBullet(&self) -> Bullet {
        match self {
            WeaponType::Pistol(bullet) => bullet.copyTemplate(),
            WeaponType::Shotgun(bullet) => bullet.copyTemplate(),
            WeaponType::Bow(arrow) => arrow.copyTemplate(),
        }
    }

    fn getType(&self) -> String {
        match self {
            WeaponType::Pistol(_) => "Pistol",
            WeaponType::Shotgun(_) => "Shotgun",
            WeaponType::Bow(_) => "Bow",
        }.to_string()
    }
}

struct Bullet {
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

    fn copyTemplate(&self) -> Self {
        Bullet::new(self.damage, self.icon)
    }

    pub fn increaseTime(&mut self) {
        self.timeAlive += 1;
    }
    
    pub fn moves(&mut self, terminal: &mut Terminal) -> bool {
        terminal.eraseBox(&self.position);
        let hasMoved = match self.direction {
            Direction::Up => self.position.moveUp(),
            Direction::Down => self.position.moveDown(),
            Direction::Left => self.position.moveLeft(),
            Direction::Right => self.position.moveRight(),
            Direction::None => false,
        };
        if hasMoved {terminal.drawChar(&self.position, self.icon)};
        hasMoved
    }

    pub fn update(&mut self, terminal: &mut Terminal) -> bool {
        self.increaseTime();
        match self.direction {
            Direction::None => {self.timeAlive = 0; return false;},
            _ => self.moves(terminal),
        }
    }

    pub fn setDiection(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn getPosition(&self) -> &Position {
        &self.position
    }

    pub fn setPosition(&mut self, pos: Position) {
        self.position = pos;
    }
}