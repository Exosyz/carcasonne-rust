use crate::side::{SideBuilder, SideKind};
use crate::tile::{TileBuilder, TileExtension};

pub trait AbbeyTileBuilder {
    fn build_A_abbey(self) -> Self;
    fn build_B_abbey(self) -> Self;
}

impl AbbeyTileBuilder for TileBuilder {
    fn build_A_abbey(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Meadow).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
            .tile_extension(TileExtension::Abbey)
    }
    fn build_B_abbey(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Meadow).build())
            .west(SideBuilder::new().kind(SideKind::Meadow).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
            .tile_extension(TileExtension::Abbey)
    }
}

pub trait RoadTileBuilder {
    fn build_U_road(self) -> Self;
    fn build_V_road(self) -> Self;
    fn build_W_road(self) -> Self;
    fn build_X_road(self) -> Self;
}

impl RoadTileBuilder for TileBuilder {
    fn build_U_road(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Meadow).build())
            .south(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
    }
    fn build_V_road(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
    }
    fn build_W_road(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Road).section(2).build())
            .south(SideBuilder::new().kind(SideKind::Road).section(3).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
    }
    fn build_X_road(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Road).section(2).build())
            .south(SideBuilder::new().kind(SideKind::Road).section(3).build())
            .east(SideBuilder::new().kind(SideKind::Road).section(4).build())
    }
}

pub trait TownTileBuilder {
    fn build_C_town(self) -> Self;
    fn build_D_town(self) -> Self;
    fn build_E_town(self) -> Self;
    fn build_F_town(self) -> Self;
    fn build_G_town(self) -> Self;
    fn build_H_town(self) -> Self;
    fn build_I_town(self) -> Self;
    fn build_J_town(self) -> Self;
    fn build_K_town(self) -> Self;
    fn build_L_town(self) -> Self;
    fn build_M_town(self) -> Self;
    fn build_N_town(self) -> Self;
    fn build_O_town(self) -> Self;
    fn build_P_town(self) -> Self;
    fn build_Q_town(self) -> Self;
    fn build_R_town(self) -> Self;
    fn build_S_town(self) -> Self;
    fn build_T_town(self) -> Self;
}

impl TownTileBuilder for TileBuilder {
    fn build_C_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .east(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .tile_extension(TileExtension::TownShield(1))
    }
    fn build_D_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Road).section(1).build())
    }
    fn build_E_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Meadow).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
    }
    fn build_F_town(self) -> Self {
        self.build_G_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_G_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Meadow).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Town).section(1).build())
    }

    fn build_H_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Meadow).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Town).section(2).build())
    }

    fn build_I_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(2).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
    }

    fn build_J_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Meadow).build())
            .south(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .east(SideBuilder::new().kind(SideKind::Road).section(1).build())
    }

    fn build_K_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Road).section(1).build())
    }

    fn build_L_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Road).section(2).build())
            .east(SideBuilder::new().kind(SideKind::Road).section(3).build())
    }

    fn build_M_town(self) -> Self {
        self.build_N_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_N_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Meadow).build())
    }

    fn build_O_town(self) -> Self {
        self.build_P_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_P_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .east(SideBuilder::new().kind(SideKind::Road).section(1).build())
    }

    fn build_Q_town(self) -> Self {
        self.build_R_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_R_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Meadow).build())
            .east(SideBuilder::new().kind(SideKind::Town).section(1).build())
    }

    fn build_S_town(self) -> Self {
        self.build_T_town()
            .tile_extension(TileExtension::TownShield(1))
    }

    fn build_T_town(self) -> Self {
        TileBuilder::new()
            .north(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .west(SideBuilder::new().kind(SideKind::Town).section(1).build())
            .south(SideBuilder::new().kind(SideKind::Road).section(1).build())
            .east(SideBuilder::new().kind(SideKind::Town).section(1).build())
    }
}
