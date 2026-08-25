# Plan del proyecto: Templo del Eclipse

## 1. Objetivo

Construir en Rust un ray caster de escritorio completo, estable y jugable. El
algoritmo de ray casting, la proyeccion de paredes, la colision, los sprites y
la logica del juego seran implementados en el proyecto; `raylib-rs` se usara
solo como capa de ventana, entrada, dibujo 2D y audio.

La entrega final debe incluir:

- repositorio de GitHub reproducible;
- instrucciones claras de compilacion y controles;
- al menos un nivel entero, cerrado y jugable;
- video corto que muestre las funciones evaluadas;
- creditos y licencias de cualquier recurso externo.

## 2. Alcance elegido

Se priorizara primero el producto obligatorio y despues todos los objetivos de
escritorio de la rubrica. No se planifica por ahora una version para hardware
especial; esa opcion depende de disponer de un dispositivo concreto y puede
introducir mucho riesgo de portabilidad.

| Objetivo | Valor maximo | Decision | Evidencia esperada |
| --- | ---: | --- | --- |
| Estetica del nivel | 30 | Si | Tema, paleta, materiales, iluminacion y HUD coherentes |
| Rotacion horizontal con mouse | 20 | Si | Cursor capturado durante el juego y giro suave |
| Disparo | 10 | Si | Arma, cooldown, impacto y objetivo susceptible al disparo |
| Minimapa en una esquina | 10 | Si | Overlay superior derecho con jugador y orientacion |
| Musica de fondo | 5 | Si | Pista propia o con licencia compatible; no Taylor Swift |
| Efectos de sonido | 10 | Si | Disparo, impacto, recoger llave y victoria |
| Sprite animado | 20 | Si | Guardian animado y/o arma con animacion |
| Pantalla de bienvenida | 5 | Si | Titulo, instrucciones y acceso al selector |
| Seleccion de multiples niveles | 10 | Si | Tres niveles seleccionables |
| Pantalla de exito | 10 | Si | Se muestra al completar la condicion del nivel |

Los objetivos planeados suman mas de 100 para reducir la dependencia del
criterio subjetivo de estetica, aunque la nota este limitada a 100.

## 3. Concepto jugable

**Templo del Eclipse** sera un laberinto ritual visto en primera persona. Cada
nivel usa una composicion distinta de piedra, obsidiana, ladrillo y muros con
glifos. El jugador debe encontrar una llave solar, neutralizar o esquivar a un
guardian y alcanzar el portal de salida.

La direccion visual buscara una escena legible antes que realismo:

- cielo y piso con gradientes oscuros;
- color o textura inequivoca para cada identificador de pared;
- sombreado por distancia y por cara para dar profundidad;
- niebla ligera para integrar los elementos lejanos;
- HUD compacto con reticula, estado de llave y municion/energia;
- portal, llave y guardian como sprites tipo billboard.

### Controles previstos

| Accion | Entrada |
| --- | --- |
| Moverse | `W`, `A`, `S`, `D` |
| Girar | Movimiento horizontal del mouse |
| Disparar | Boton izquierdo |
| Pausar/liberar cursor | `Esc` |
| Reiniciar nivel | `R` |
| Confirmar menu | `Enter` o clic |

## 4. Diseno tecnico

### Biblioteca base

- Rust, edicion 2024.
- `raylib-rs` 6.0 para ventana, teclado/mouse, dibujo, texturas y audio.
- Sin motor 3D ni implementacion externa de ray casting.
- Resolucion inicial: 960 x 540, 60 FPS objetivo y tiempo de cuadro (`dt`)
  limitado para evitar saltos de movimiento tras una pausa.

### Representacion del mapa

Los niveles se almacenaran como mapas de texto incluidos en el ejecutable.
Una celda representa una unidad del mundo:

- `1` a `5`: materiales de pared diferentes;
- `.`: espacio transitable;
- `S`: posicion inicial;
- `K`: llave;
- `E`: portal de salida;
- `G`: guardian.

El cargador comprobara dimensiones, borde completamente cerrado, simbolos
validos y una unica posicion inicial. Una coordenada fuera del mapa siempre se
tratara como pared, nunca se indexara directamente sin validacion.

### Ray casting

Se usara DDA (Digital Differential Analyzer), un rayo por columna:

1. calcular direccion a partir del vector del jugador y el plano de camara;
2. avanzar entre celdas hasta encontrar un material solido;
3. obtener distancia perpendicular para evitar efecto ojo de pez;
4. proyectar la altura de la pared y limitarla a la pantalla;
5. seleccionar color/textura por material y oscurecer segun cara/distancia;
6. guardar la distancia de la columna en un `z-buffer` para ocultar sprites.

### Movimiento y colision

El jugador sera un circulo, no un punto. La posicion candidata se resolvera por
separado en X y Y para permitir deslizarse por las paredes. Se comprobaran los
puntos extremos del radio contra celdas solidas y el movimiento se subdividira
si un cuadro excepcionalmente largo pudiera atravesar una pared.

### Sprites, disparo y condicion de victoria

Los sprites se ordenaran de lejos a cerca, se proyectaran como billboards y se
compararan con el `z-buffer`. El guardian alternara cuadros de animacion. El
disparo sera hitscan desde el centro de la pantalla, con cooldown, fogonazo,
sonido e impacto. La victoria requiere recoger la llave y entrar al portal; el
guardian puede actuar como obstaculo/objetivo, sin que su IA sea necesaria para
la primera version jugable.

### Estados de la aplicacion

```text
Bienvenida -> Selector de nivel -> Jugando <-> Pausa
                                      |
                                      v
                                    Exito
                                      |
                         Reintentar / Selector / Salir
```

### Estructura prevista

```text
assets/
  audio/
  textures/
levels/
  eclipse_1.txt
  eclipse_2.txt
  eclipse_3.txt
src/
  main.rs
  app.rs
  audio.rs
  game/
    collision.rs
    entities.rs
    level.rs
    mod.rs
    player.rs
    raycast.rs
  render/
    minimap.rs
    mod.rs
    sprites.rs
    ui.rs
    world.rs
  screens/
    menu.rs
    mod.rs
    victory.rs
tests/
README.md
CREDITS.md
```

La estructura podra compactarse si algun modulo resulta artificialmente
pequeno. La logica matematica se mantendra separada de raylib para poder
probarla sin abrir una ventana.

## 5. Fases de trabajo

### Fase 0: base reproducible

- normalizar el nombre del paquete;
- agregar `raylib-rs` y comprobar que abre una ventana;
- crear configuracion, bucle principal y estados vacios;
- agregar `README.md`, `.gitignore` y estructura inicial.

**Criterio de salida:** `cargo run` abre y cierra correctamente; `cargo test`,
`cargo fmt --check` y `cargo clippy` pasan.

### Fase 1: nucleo del ray caster

- cargar y validar un mapa cerrado;
- implementar jugador, camara, DDA, distancia perpendicular y paredes;
- dibujar techo/piso y materiales con colores distintos;
- agregar pruebas unitarias para mapa y rayos conocidos.

**Criterio de salida:** el nivel entero se renderiza, no hay accesos fuera de
rango y cada tipo de pared se distingue con claridad.

### Fase 2: navegacion segura

- movimiento con `WASD` dependiente de `dt`;
- rotacion horizontal con mouse y pausa que libera el cursor;
- colision circular con deslizamiento por ejes;
- minimapa en la esquina, con posicion y direccion del jugador.

**Criterio de salida:** se puede recorrer el mapa sin atravesar paredes, incluso
en esquinas y con variaciones bruscas de tiempo de cuadro.

### Fase 3: ciclo jugable

- llave, portal, guardian y sprites con oclusion;
- disparo hitscan, cooldown y respuesta visual;
- animacion de guardian/arma;
- condicion de victoria y pantalla de exito.

**Criterio de salida:** el nivel tiene inicio, objetivo, riesgo/obstaculo y final
demostrable, sin bloqueos al reiniciar.

### Fase 4: menus y multiples niveles

- pantalla de bienvenida;
- selector de tres niveles;
- reinicio y regreso al menu;
- validar que cada mapa sea completable.

**Criterio de salida:** todos los estados se pueden recorrer repetidamente sin
reiniciar el programa y los tres niveles se cargan de forma segura.

### Fase 5: audio y direccion artistica

- musica ambiental en bucle;
- efectos de disparo, impacto, llave, portal y victoria;
- texturas o patrones por pared, sombras, niebla, HUD y transiciones;
- registrar origen y licencia de cada recurso.

**Criterio de salida:** volumen equilibrado, recursos ausentes manejados sin
panic y presentacion visual coherente.

### Fase 6: aseguramiento y entrega

- pruebas automatizadas de DDA, colision, parser y condicion de victoria;
- prueba manual de menus, mouse, bordes, esquinas y los tres niveles;
- medir estabilidad y fluidez en una compilacion `--release`;
- completar README con requisitos, instalacion, controles y rubrica;
- preparar guion y grabar un video de 60 a 90 segundos;
- publicar una version etiquetada en GitHub.

**Criterio de salida:** otra persona puede clonar, compilar, jugar y verificar
los objetivos usando solamente las instrucciones del repositorio.

## 6. Estrategia de pruebas

### Automatizadas

- un rayo frontal devuelve material, lado y distancia esperados;
- rayos paralelos a los ejes no producen divisiones invalidas;
- posiciones fuera de limites se consideran solidas;
- el jugador no penetra una pared frontal ni una esquina;
- el parser rechaza mapas abiertos, irregulares o sin inicio;
- la salida no se activa sin llave y si se activa con llave;
- los cambios de nivel reinician todo el estado transitorio.

### Manuales

- mantener movimiento contra cada pared y esquina durante varios segundos;
- pausar, cambiar el foco de ventana y continuar;
- girar rapidamente con el mouse sin saltos verticales;
- disparar repetidamente y verificar cooldown/animacion/audio;
- confirmar que el minimapa permanezca superpuesto en una esquina;
- completar y reiniciar cada nivel varias veces;
- ejecutar sin audio disponible y comprobar que no se cierre inesperadamente.

## 7. Riesgos y mitigaciones

| Riesgo | Mitigacion |
| --- | --- |
| Atravesar paredes con `dt` alto | Limitar `dt`, usar radio y subdividir movimiento |
| Panic por mapa o rayo fuera de rango | Bordes validados y consultas seguras que tratan el exterior como solido |
| Distorsion ojo de pez | Usar distancia perpendicular, no longitud directa del rayo |
| Sprites visibles a traves de muros | `z-buffer` por columna |
| Dependencias nativas de raylib | Verificar compilacion en Fase 0 y documentar paquetes requeridos |
| Recursos con licencia dudosa | Preferir recursos propios/CC0 y mantener `CREDITS.md` |
| Mucho pulido antes de tener juego | Cada fase termina en un incremento ejecutable y verificable |
| Criterio estetico subjetivo | Exceder 100 puntos planeados y mantener una direccion visual consistente |

## 8. Definicion de terminado

El proyecto estara terminado cuando:

- cumple todas las condiciones obligatorias sin crashes conocidos;
- ofrece tres niveles completos con paredes diferenciadas;
- implementa los objetivos marcados en la tabla de alcance;
- todas las comprobaciones automaticas pasan;
- la prueba manual se completa en compilacion `--release`;
- README, creditos, capturas/video y enlace de GitHub estan listos.

## 9. Proximo incremento

La siguiente sesion de implementacion debe cubrir solamente la **Fase 0** y el
esqueleto minimo de la **Fase 1**: ventana, bucle, un mapa validado y las
estructuras matematicas del jugador/rayo. No se incorporaran recursos visuales
o audio antes de verificar esa base.
