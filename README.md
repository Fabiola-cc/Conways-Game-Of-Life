# Game of Life en Rust

Este proyecto implementa el juego de la vida de Conway usando el lenguaje de programación Rust. El juego de la vida es un autómata celular desarrollado por el matemático John Conway.

## Descripción

El juego de la vida es un autómata celular de cero jugadores, lo que significa que su evolución está determinada por su estado inicial, sin requerir ninguna entrada adicional. El universo del juego es una cuadrícula de celdas que pueden estar vivas o muertas. Las celdas evolucionan en pasos discretos, siguiendo un conjunto simple de reglas:

1. Cualquier célula viva con menos de dos células vivas vecinas muere, como si por soledad.
2. Cualquier célula viva con dos o tres células vivas vecinas permanece viva para la siguiente generación.
3. Cualquier célula viva con más de tres células vivas vecinas muere, como si por sobrepoblación.
4. Cualquier célula muerta con exactamente tres células vivas vecinas se convierte en una célula viva.

## Características

- Implementación en Rust.
- Interfaz gráfica utilizando la biblioteca `minifb`.
- Soporte para varios patrones iniciales predefinidos (osciladores, naves espaciales, etc.).
- Opciones de color personalizables para las células vivas y muertas.

## Actualmente
- El resultado que se configuró en esta ocasión es el siguiente:
![GIF del Resultado](https://github.com/Fabiola-cc/Conways-Game-Of-Life/blob/main/Lab2_output.gif)
