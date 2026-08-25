# Templo del Eclipse

Proyecto 1 de Graficas por Computadora: un ray caster jugable implementado en
Rust. El proyecto se construye incrementalmente en fases verificables.

## Estado actual

**Fase 2 en progreso (parte 1):** el ray caster ya permite movimiento con
`WASD`, velocidad independiente de los FPS y colision circular con deslizamiento
por paredes. La rotacion con mouse y el minimapa se agregaran en los siguientes
incrementos de esta fase.

## Requisitos

- Rust estable con Cargo.
- Un compilador de C y CMake para compilar raylib.
- Bibliotecas de desarrollo de ventana, OpenGL y audio correspondientes al
  sistema operativo.

El proyecto utiliza `raylib-rs` como capa de ventana, entrada, dibujo y audio.
El algoritmo de ray casting se implementa dentro de este repositorio.

## Ejecutar

```bash
cargo run
```

Para una compilacion optimizada:

```bash
cargo run --release
```

## Controles disponibles en la base

| Pantalla | Control | Accion |
| --- | --- | --- |
| Bienvenida | `Enter` | Abrir selector de nivel |
| Bienvenida/selector | `Q` | Salir |
| Selector | Flechas izquierda/derecha | Elegir nivel (uno disponible) |
| Selector | `Enter` | Iniciar nivel |
| Juego | `W`, `A`, `S`, `D` | Moverse y desplazarse lateralmente |
| Juego | `R` | Regresar al inicio del nivel |
| Juego | `Esc` | Pausar |
| Juego | `V` | Probar pantalla de victoria |
| Pausa | `Esc` | Continuar |
| Pausa | `M` | Volver al selector |
| Victoria | `Enter` | Volver al selector |

La rotacion horizontal con mouse se habilitara en la parte 2 de esta fase y el
disparo se incorporara en la Fase 3.

## Comprobaciones de desarrollo

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Estructura actual

- `src/main.rs`: inicializacion de raylib y bucle principal.
- `src/config.rs`: configuracion global de ventana y simulacion.
- `src/app.rs`: estado y transiciones de la aplicacion.
- `src/game/collision.rs`: colision circular, subpasos y deslizamiento por ejes.
- `src/game/level.rs`: parser y validacion del mapa.
- `src/game/raycast.rs`: algoritmo DDA y resultado de cada rayo.
- `src/game/player.rs`: posicion, direccion y plano de camara.
- `src/render/world.rs`: proyeccion y coloreado de paredes, techo y piso.
- `src/screens/`: presentacion de las pantallas y HUD provisional.
- `levels/`: mapas de texto incluidos en el ejecutable.

## Entrega

La version final incluira instrucciones completas, creditos de recursos, enlace
al video demostrativo y una lista de los objetivos de la rubrica implementados.
