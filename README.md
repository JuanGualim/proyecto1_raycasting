# Templo del Eclipse

Proyecto 1 de Graficas por Computadora: un ray caster jugable implementado en
Rust. El proyecto se construye incrementalmente en fases verificables.

## Estado actual

**Fase 4.2 completada:** el juego incluye tres niveles completos y seleccionables.
Cada camara tiene una distribucion, dificultad y recorrido propios, conservando
el ciclo de guardian, llave y portal.

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
| Selector | Flechas izquierda/derecha | Recorrer los niveles disponibles |
| Selector | `Enter` | Iniciar nivel |
| Juego | `W`, `A`, `S`, `D` | Moverse y desplazarse lateralmente |
| Juego | Movimiento horizontal del mouse | Girar la camara |
| Juego | Clic izquierdo | Disparar |
| Juego | `R` | Reiniciar posicion y combate |
| Juego | `Esc` | Pausar |
| Pausa | `Esc` | Continuar |
| Pausa | `M` | Volver al selector |
| Victoria | `Enter` | Volver al selector |

La condicion de victoria requiere eliminar al guardian, recoger la llave y
alcanzar el portal.

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
- `src/game/catalog.rs`: catalogo, metadatos y carga segura de niveles incluidos.
- `src/game/combat.rs`: interseccion hitscan y resultados de disparo.
- `src/game/entities.rs`: tipos y estado base de llave, portal y guardian.
- `src/game/level.rs`: parser y validacion del mapa.
- `src/game/objective.rs`: eventos y mensajes del objetivo del nivel.
- `src/game/raycast.rs`: algoritmo DDA y resultado de cada rayo.
- `src/game/player.rs`: posicion, direccion y plano de camara.
- `src/render/minimap.rs`: minimapa superpuesto y proyeccion del jugador.
- `src/render/palette.rs`: colores compartidos por paredes, HUD y minimapa.
- `src/render/sprites.rs`: proyeccion billboard y oclusion de entidades.
- `src/render/world.rs`: proyeccion y coloreado de paredes, techo y piso.
- `src/screens/`: presentacion de las pantallas y HUD provisional.
- `levels/`: mapas de texto incluidos en el ejecutable.

## Verificacion manual de navegacion

1. Mantener movimiento contra paredes y esquinas sin atravesarlas.
2. Recorrer pasillos mientras se gira rapidamente con el mouse.
3. Confirmar que el minimapa siga la posicion y orientacion del jugador.
4. Pausar con `Esc`, comprobar que el cursor quede libre y continuar.
5. Reiniciar con `R` y verificar el regreso a la posicion y orientacion inicial.

## Verificacion manual del ciclo jugable

1. Disparar tres veces al guardian y confirmar su eliminacion.
2. Recorrer el nivel hasta la llave amarilla y recogerla por proximidad.
3. Confirmar en el HUD y minimapa que el portal cambia de bloqueado a activo.
4. Entrar al portal y comprobar la pantalla de exito con el tiempo empleado.
5. Volver al selector y verificar que el nivel se reinicie completamente.

## Verificacion manual de multiples niveles

1. Recorrer las tres opciones del selector con las flechas izquierda y derecha.
2. Entrar a cada nivel y confirmar que el mapa, el punto inicial y las entidades
   cambien.
3. Completar al menos una vez el Santuario de Obsidiana y la Cripta del Musgo.
4. Regresar al selector desde pausa y confirmar que se pueda elegir otra camara.

## Entrega

La version final incluira instrucciones completas, creditos de recursos, enlace
al video demostrativo y una lista de los objetivos de la rubrica implementados.
