# Meowl 🦉

Petit compagnon de bureau Linux qui compte les mots que vous tapez. Meowl s'installe en bas à droite de votre écran, reste au-dessus de toutes les fenêtres et s'agite joyeusement à chaque mot.

![Meowl](meowl.png)

## Fonctionnalités

- Reste **toujours au premier plan** sur toutes les fenêtres
- **Visible sur tous les bureaux virtuels**
- **Ignoré par la barre des tâches** (et Alt-Tab)
- Compte les mots tapés sur **n'importe quel clavier** du système via `evdev`
- Persiste le compteur entre les sessions
- Le sprite **frétille** à chaque nouveau mot

## Pile technique

- [Rust](https://www.rust-lang.org/) + [Tauri 2](https://v2.tauri.app/)
- [Svelte](https://svelte.dev/) pour l'interface
- [`evdev`](https://crates.io/crates/evdev) pour la capture clavier globale

## Pré-requis

- Linux (testé sur **KDE Plasma 6**, sessions X11 et Wayland)
- `pnpm`, `cargo`, toolchain Rust
- Les dépendances système habituelles de Tauri (`webkit2gtk`, `gtk3`, etc.)
- XWayland doit être installé si vous utilisez une session Wayland (généralement inclus par défaut sur KDE Plasma)
- Votre utilisateur doit appartenir au groupe `input` pour lire les événements clavier :

```sh
sudo usermod -aG input $USER
# puis se déconnecter / reconnecter
```

## Démarrage

```sh
pnpm install
pnpm tauri dev
```

Pour produire un binaire optimisé :

```sh
pnpm tauri build
```

## Notes de compatibilité

- **X11 et Wayland** : Meowl utilise les APIs natives de Tauri (`set_always_on_top`, `set_skip_taskbar`, `set_visible_on_all_workspaces`) qui s'appuient sur les hints `_NET_WM_*`. Sur les sessions Wayland, le binaire force `GDK_BACKEND=x11` au démarrage pour passer par XWayland — KWin honore alors les hints comme sur une vraie session X11. Le support Wayland natif (`wlr-layer-shell`) n'est pas implémenté ; XWayland est suffisant pour un compagnon de bureau.
- **Transparence WebKitGTK** : sur les versions récentes de WebKitGTK, le rendu DMA-BUF empêche l'affichage des fenêtres transparentes. Le binaire force donc `WEBKIT_DISABLE_DMABUF_RENDERER=1` au démarrage.
- **Positionnement** : Meowl est ancré à 20 px du bord droit et 80 px du bord bas du moniteur courant.

## Licence

[MIT](LICENSE)
