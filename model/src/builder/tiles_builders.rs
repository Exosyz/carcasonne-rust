use crate::side::SideKind;
use crate::tile::{TileBuilder, TileExtension};

pub trait AbbeyTileBuilder {
    fn build_A_abbey(&mut self) -> &mut Self;
    fn build_B_abbey(&mut self) -> &mut Self;
}

impl AbbeyTileBuilder for TileBuilder {
    fn build_A_abbey(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
            .tile_extension(TileExtension::Abbey)
    }
    fn build_B_abbey(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
            .tile_extension(TileExtension::Abbey)
    }
}

pub trait RoadTileBuilder {
    fn build_U_road(&mut self) -> &mut Self;
    fn build_V_road(&mut self) -> &mut Self;
    fn build_W_road(&mut self) -> &mut Self;
    fn build_X_road(&mut self) -> &mut Self;
}

impl RoadTileBuilder for TileBuilder {
    fn build_U_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_V_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_W_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(2))
            .south(|s| s.kind(SideKind::Road).section(3))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_X_road(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Road).section(1))
            .west(|s| s.kind(SideKind::Road).section(2))
            .south(|s| s.kind(SideKind::Road).section(3))
            .east(|s| s.kind(SideKind::Road).section(4))
    }
}

pub trait TownTileBuilder {
    fn build_C_town(&mut self) -> &mut Self;
    fn build_D_town(&mut self) -> &mut Self;
    fn build_E_town(&mut self) -> &mut Self;
    fn build_F_town(&mut self) -> &mut Self;
    fn build_G_town(&mut self) -> &mut Self;
    fn build_H_town(&mut self) -> &mut Self;
    fn build_I_town(&mut self) -> &mut Self;
    fn build_J_town(&mut self) -> &mut Self;
    fn build_K_town(&mut self) -> &mut Self;
    fn build_L_town(&mut self) -> &mut Self;
    fn build_M_town(&mut self) -> &mut Self;
    fn build_N_town(&mut self) -> &mut Self;
    fn build_O_town(&mut self) -> &mut Self;
    fn build_P_town(&mut self) -> &mut Self;
    fn build_Q_town(&mut self) -> &mut Self;
    fn build_R_town(&mut self) -> &mut Self;
    fn build_S_town(&mut self) -> &mut Self;
    fn build_T_town(&mut self) -> &mut Self;
}

impl TownTileBuilder for TileBuilder {
    fn build_C_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Town).section(1))
            .east(|s| s.kind(SideKind::Town).section(1))
            .tile_extension(TileExtension::TownShield(1))
    }
    fn build_D_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Road).section(1))
    }
    fn build_E_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }
    fn build_F_town(&mut self) -> &mut Self {
        self.build_G_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_G_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(1))
    }

    fn build_H_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Meadow))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(2))
    }

    fn build_I_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(2))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }

    fn build_J_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Meadow))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_K_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_L_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Road).section(1))
            .south(|s| s.kind(SideKind::Road).section(2))
            .east(|s| s.kind(SideKind::Road).section(3))
    }

    fn build_M_town(&mut self) -> &mut Self {
        self.build_N_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_N_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Meadow))
    }

    fn build_O_town(&mut self) -> &mut Self {
        self.build_P_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_P_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Road).section(1))
    }

    fn build_Q_town(&mut self) -> &mut Self {
        self.build_R_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_R_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Meadow))
            .east(|s| s.kind(SideKind::Town).section(1))
    }

    fn build_S_town(&mut self) -> &mut Self {
        self.build_T_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_T_town(&mut self) -> &mut Self {
        self.north(|s| s.kind(SideKind::Town).section(1))
            .west(|s| s.kind(SideKind::Town).section(1))
            .south(|s| s.kind(SideKind::Road).section(1))
            .east(|s| s.kind(SideKind::Town).section(1))
    }
}
