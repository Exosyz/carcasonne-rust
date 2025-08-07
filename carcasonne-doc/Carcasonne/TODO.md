
- [ ] Pour le tile renderer
	- [ ] Comment compute le centre : 
		- [ ] lui qui a le centre en 3x3 (center_size)
		- [ ] Priorité sur les villages
			- [ ] Si village.len() > 2
				- [ ] Route qui contourne de 1 ou en zigzag
			- [ ] Sion route
				- [ ] Trouver un layout cool pour les gerer
	- [ ] Pour les TileEnhancement prendre une position random de la liste et changer par un autre char
	- [ ] Charactère
		- [ ] Pour les routes qui ne se touche pas, ajouter un carrés vide : ◻
		- [ ] pour la ville : █
		- [ ] pour les shield : ■
		- [ ] pour les routes

| Nom                 | Symbole | Unicode | Description                       |
| ------------------- | ------- | ------- | --------------------------------- |
| Horizontal          | `═`     | U+2550  | Trait horizontal double           |
| Vertical            | `║`     | U+2551  | Trait vertical double             |
| ┌ coin haut gauche  | `╔`     | U+2554  | Angle haut gauche                 |
| ┐ coin haut droit   | `╗`     | U+2557  | Angle haut droit                  |
| └ coin bas gauche   | `╚`     | U+255A  | Angle bas gauche                  |
| ┘ coin bas droit    | `╝`     | U+255D  | Angle bas droit                   |
| ├ T-junction droite | `╠`     | U+2560  | Connexion gauche vers haut/bas    |
| ┤ T-junction gauche | `╣`     | U+2563  | Connexion droite vers haut/bas    |
| ┬ T-junction bas    | `╦`     | U+2566  | Connexion haut vers gauche/droite |
| ┴ T-junction haut   | `╩`     | U+2569  | Connexion bas vers gauche/droite  |
| ┼ croix             | `╬`     | U+256C  | Connexion dans les 4 directions   |

- [ ] 
	- [ ] Trouver une solution pour les routes horizontale
	- [ ] Ajouter des tests pour 3/7/9
	      3 a voir peut etre pas utile
