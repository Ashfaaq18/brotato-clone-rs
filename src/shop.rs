#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UpgradeKind {
    Heal,
    Speed,
    Damage,
    FireRate,
}

pub struct Upgrade {
    pub kind: UpgradeKind,
    pub name: &'static str,
    pub description: &'static str,
    pub cost: i32,
}

pub const UPGRADES: &[Upgrade] = &[
    Upgrade {
        kind: UpgradeKind::Heal,
        name: "Heal",
        description: "+30 HP",
        cost: 3,
    },
    Upgrade {
        kind: UpgradeKind::Speed,
        name: "Speed",
        description: "+15 move speed",
        cost: 4,
    },
    Upgrade {
        kind: UpgradeKind::Damage,
        name: "Damage",
        description: "+10 projectile damage",
        cost: 5,
    },
    Upgrade {
        kind: UpgradeKind::FireRate,
        name: "Fire Rate",
        description: "+1 shot/sec",
        cost: 6,
    },
];
