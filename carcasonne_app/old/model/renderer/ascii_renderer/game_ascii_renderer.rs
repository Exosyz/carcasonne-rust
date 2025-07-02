use crate::game::Game;
use crate::renderer::ascii_renderer::AsciiRenderer;
use crate::renderer::base_renderer::{RenderOutput, TileRenderer};
use crate::renderer::game_renderer::GameRenderer;

impl GameRenderer for AsciiRenderer {
    fn render(&self, game: &mut Game) {
        while let Some(tile) = game.get_next_tile() {
            //TODO Rotate tile
            let rotated_tile = tile;

            let tile_output = <AsciiRenderer as TileRenderer>::render(self, &rotated_tile);

            println!("{:#?}", tile);
            match tile_output {
                RenderOutput::Ascii(ascii_art) => {
                    for row in ascii_art {
                        for c in row {
                            print!("{}", c);
                        }
                        println!();
                    }
                    println!("=============================");
                }
                _ => panic!("Expected ASCII output"),
            }
        }
    }
}
