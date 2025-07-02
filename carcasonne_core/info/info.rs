// Direction enum for edges
enum Direction {
    North,
    South,
    East,
    West,
}

// Trait for anything that can render itself on a 5x5 char grid
trait RenderableFeature {
    // Given a mutable 5x5 char grid, draw this feature onto it
    fn render(&self, grid: &mut [[char; 5]; 5]);
}

// Trait for rotation behavior (optional but useful)
trait Rotatable {
    // Rotate the feature clockwise by 90 degrees * times
    fn rotate(&mut self, times: usize);
}

// Example feature structs implementing behaviors
struct City {
    edges: Vec<Direction>, // Which edges city touches
}
impl RenderableFeature for City {
    fn render(&self, grid: &mut [[char; 5]; 5]) {
        // Fill city cells with 'C' respecting edges
    }
}
impl Rotatable for City {
    fn rotate(&mut self, times: usize) {
        // Rotate edges directions accordingly
    }
}

struct Road {
    edges: Vec<Direction>, // Road connections
}
impl RenderableFeature for Road {
    fn render(&self, grid: &mut [[char; 5]; 5]) {
        // Draw road paths 'R'
    }
}
impl Rotatable for Road {
    fn rotate(&mut self, times: usize) {
        // Rotate road edges
    }
}

struct Cloister;
impl RenderableFeature for Cloister {
    fn render(&self, grid: &mut [[char; 5]; 5]) {
        // Put 'X' in center cell [2][2]
        grid[2][2] = 'X';
    }
}
impl Rotatable for Cloister {
    fn rotate(&mut self, _times: usize) {
        // Cloister rotation does nothing or no-op
    }
}

// Meeple struct to mark placement on tile
struct Meeple {
    owner_id: usize,
    position: (usize, usize),
    symbol: char, // e.g., 'M'
}

// Main Tile struct holding features and optional meeple
struct Tile {
    features: Vec<Box<dyn RenderableFeature>>,
}

enum RenderSymbol {
    Char(char),
    Unicode(String),
    Sprite(u32),
}

trait RenderableFeature {
    fn render(&self) -> Vec<(Position, RenderSymbol)>;
}

trait Renderer {
    fn draw_tile(&self, tile_draws: &[(Position, RenderSymbol)]);
}

struct TextRenderer;

impl Renderer for TextRenderer {
    fn draw_tile(&self, tile_draws: &[(Position, RenderSymbol)]) {
        // For each position, print ascii char or '?'
    }
}

struct UnicodeRenderer;

impl Renderer for UnicodeRenderer {
    fn draw_tile(&self, tile_draws: &[(Position, RenderSymbol)]) {
        // Use unicode or fallback
    }
}

// Builkder

struct TileBuilder {
    features: Vec<Box<dyn RenderableFeature>>,
    meeple: Option<Meeple>,
    rotation: usize,
}

impl TileBuilder {
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
            meeple: None,
            rotation: 0,
        }
    }

    pub fn add_city(mut self, edges: &[Direction]) -> Self {
        let city = City {
            edges: edges.to_vec(),
        };
        self.features.push(Box::new(city));
        self
    }

    pub fn add_road(mut self, edges: &[Direction]) -> Self {
        let road = Road {
            edges: edges.to_vec(),
        };
        self.features.push(Box::new(road));
        self
    }

    pub fn add_cloister(mut self) -> Self {
        self.features.push(Box::new(Cloister));
        self
    }

    pub fn with_meeple(mut self, owner_id: usize, pos: (usize, usize), symbol: char) -> Self {
        self.meeple = Some(Meeple {
            owner_id,
            position: pos,
            symbol,
        });
        self
    }

    pub fn with_rotation(mut self, rotation: usize) -> Self {
        self.rotation = rotation % 4;
        self
    }

    pub fn build(self) -> Tile {
        Tile {
            features: self.features,
            meeple: self.meeple,
            rotation: self.rotation,
        }
    }
}

//Factory
struct TileFactory;

impl TileFactory {
    pub fn city_north() -> Tile {
        TileBuilder::new().add_city(&[Direction::North]).build()
    }

    pub fn city_north_east() -> Tile {
        TileBuilder::new()
            .add_city(&[Direction::North, Direction::East])
            .build()
    }

    pub fn road_south() -> Tile {
        TileBuilder::new().add_road(&[Direction::South]).build()
    }

    pub fn cloister_center() -> Tile {
        TileBuilder::new().add_cloister().build()
    }

    // Ajoute d’autres presets selon besoin...

    pub fn standard_tile_1() -> Tile {
        TileBuilder::new()
            .add_city(&[Direction::North, Direction::West])
            .add_road(&[Direction::South])
            .build()
    }
}

/*
| Pattern   | Usage principal                 | Avantages                      |
| --------- | ------------------------------- | ------------------------------ |
| State     | Phases du jeu                   | Code clair, évite gros if/else |
| Command   | Actions joueur                  | Undo, replay, réseau           |
| Observer  | Communication moteur → UI       | Découplage, évènements         |
| Builder   | Construction d’objets complexes | API fluide, sécurité           |
| Flyweight | Partage d’objets immuables      | Optimisation mémoire           |
| Composite | Gestion de groupes liés         | Facilité calcul zones          |
| Strategy  | Comportement variable           | Modularité, testabilité        |
| ECS       | Architecture complète du jeu    | Scalabilité, modularité        |


1. State Pattern — gérer les phases du jeu
Carcassonne a plusieurs étapes (poser tuile, placer meeple, calculer score, etc).
Le pattern State t’aide à organiser ces phases comme des états distincts avec des règles spécifiques.

Avantage : isoler la logique par phase, réduire conditions complexes.

En Rust, tu peux utiliser des enums avec data ou des structs qui implémentent un trait GameState.

Transition entre états via méthodes comme next_state().

2. Command Pattern — enregistrer et rejouer les
Pour gérer des actions joueur (poser tuile, poser meeple, passer tour), tu peux encapsuler chaque action dans une commande.

Avantage : facilite l’annulation, rejouabilité, gestion du réseau.

En Rust, modélise des structs/enum Command avec execute() et undo().

Utile pour un mode replay ou undo.

3. Observer Pattern — mise à jour UI / évènements
Pour que l’interface graphique ou le terminal réagisse aux changements du jeu sans être couplée au carcasonne_core, utilise un système d’évènements.

Avantage : découplage entre moteur et UI.

Implémentation simple avec des callbacks, channels ou crates comme event-bus.

Le moteur émet des événements (TilePlaced, ScoreUpdated) que l’UI écoute.

4. Builder Pattern — pour la construction de tuiles (comme déjà fait)
Tu l’as bien mis en place pour créer tes tuiles étape par étape, avec validation.

5. Flyweight Pattern — optimisation mémoire des tuiles
Dans Carcassonne, beaucoup de tuiles identiques sont utilisées.
Avec Flyweight, tu peux stocker un seul modèle de tuile et référencer plusieurs fois cette instance dans le plateau.

Avantage : moins d’allocation, économie mémoire.

En Rust, utilise des références (Arc<Tile>) pour partager les tuiles immuables.

6. Composite Pattern — modéliser des groupes de features ou zones
Pour gérer des zones contiguës (ex : ville connectée, route continue), le pattern Composite permet de traiter un groupe d’éléments comme un seul.

Utile pour le scoring et la gestion des meeples.

En Rust, tu peux modéliser ça avec des structs récursifs ou graphes.

7. Strategy Pattern — varier le comportement (IA, rendu, règles)
Par exemple, différentes stratégies d’IA ou différentes façons de rendre les tuiles.

Tu définis un trait et plusieurs implémentations interchangeables.

8. Entity-Component-System (ECS) — architecture de jeu
Si tu veux faire évoluer ton jeu vers quelque chose de plus complexe (animations, effets), un ECS permet de séparer entités, composants et systèmes.

Rust a plusieurs crates ECS (ex : specs, bevy_ecs).

Plus complexe, peut être overkill pour un Carcassonne classique.



*/
