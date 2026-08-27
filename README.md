# Templo del Eclipse

Proyecto 1 de Graficas por Computadora: un ray caster jugable implementado en
Rust. El proyecto se construye incrementalmente en fases verificables.

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
| Todas | `F1` | Activar o silenciar el audio |
| Selector | Flechas izquierda/derecha o `A`/`D` | Recorrer los niveles disponibles |
| Selector | `Enter` | Iniciar nivel |
| Selector | `Esc` | Volver a la bienvenida |
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
cargo test --release
cargo build --release
```

## Estructura actual

- `src/main.rs`: inicializacion de raylib y bucle principal.
- `src/audio.rs`: sintesis, codificacion WAV y reproduccion de musica original.
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
- `src/screens/`: bienvenida, selector visual, HUD, pausa y pantalla de exito.
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

## Verificacion manual del selector

1. Abrir el selector desde la bienvenida y comprobar las tres fichas de nivel.
2. Verificar que nombre, dificultad, descripcion y plano cambien juntos.
3. Confirmar que las flechas y `A`/`D` recorran el catalogo de forma circular.
4. Presionar `Esc` para volver a la bienvenida y `Enter` para regresar.


## Audio

La musica de fondo y los efectos son composiciones originales sintetizadas en
tiempo de ejecucion. No utilizan canciones ni recursos externos, por lo que no
requieren atribuciones adicionales.

Los efectos distinguen movimiento y confirmacion de menu, disparo, impacto,
guardian derrotado, llave recogida, portal activado y victoria. Una cola de
eventos garantiza que cada efecto corresponda a una accion concreta.

## Verificacion manual del audio

1. Usar `F1` para silenciar y restaurar todo el audio.
2. Cambiar y confirmar niveles para escuchar los dos efectos del menu.
3. Disparar al aire y al guardian para distinguir disparo, impacto y derrota.
4. Recoger la llave y confirmar el sonido de activacion cuando el portal quede
   listo.
5. Cruzar el portal y escuchar el efecto de victoria.


## Objetivos de la rubrica

| Objetivo | Estado | Evidencia |
| --- | --- | --- |
| Nivel entero y jugable | Cumplido | Tres mapas cerrados con guardian, llave y portal alcanzables |
| Colisiones seguras | Cumplido | Radio del jugador, movimiento por subpasos y deslizamiento por ejes |
| Paredes diferenciadas | Cumplido | Cinco materiales con colores distintos en todos los niveles |
| Rotacion horizontal con mouse | Cumplido | Captura del cursor y rotacion solo sobre el eje horizontal |
| Disparo | Cumplido | Hitscan con enfriamiento, oclusion por paredes y retroalimentacion visual |
| Minimapa en una esquina | Cumplido | Superpuesto en la esquina superior derecha |
| Musica de fondo | Cumplido | Composicion original reproducida en bucle |
| Efectos de sonido | Cumplido | Ocho efectos vinculados a eventos del juego |
| Animacion de sprites | Cumplido | Guardian, llave y portal utilizan animacion por cuadros |
| Pantalla de bienvenida | Cumplido | Portada animada antes del selector |
| Seleccion de multiples niveles | Cumplido | Catalogo visual con tres niveles |
| Pantalla de exito | Cumplido | Se activa al derrotar al guardian, recoger la llave y cruzar el portal |


## Entrega

- Repositorio: [github.com/JuanGualim/proyecto1_raycasting](https://github.com/JuanGualim/proyecto1_raycasting)
- Video demostrativo: [video_proyecto1](https://youtu.be/2oYrvrAxKTk)

