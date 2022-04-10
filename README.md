# Ejercicio 2 - diff

Encontrar la diferencia entre dos archivos es un problema que es
bastante conocido y estudiado.

La mayoría de las implementaciones usan el algoritmo de Myers, en este
ejercicio, haremos que calcule la subsecuencia común más larga entre
los dos archivos con el algoritmo LCS y use esa información para
calcular su diferencia.

Este ejercicio se divide en hitos a cumplir.

## Leer los dos archivos en dos vectores de líneas

En este hito, se debe implementar la función `read_file_lines` la cual
debe tomar como parámetro la ruta al archivo y devolver un vector
conteniendo las líneas del archivo.

## Implementar el algoritmo LCS - Longest Common Subsequence

*Longest Common Subsequence* es un algoritmo conocido: dadas dos
secuencias, ¿cuál es la subsecuencia más larga que aparece en ambas?

Si las secuencias de caracteres son `a b c d` y `a d b c`, la subsecuencia
común más larga es `a b c`, porque estos caracteres aparecen en ambas
secuencias en ese orden (notar que la subsecuencia no necesita ser
consecutiva, sino que debe estar en orden).

Cuando se hace el diff entre dos archivos, queremos determinar cuáles
lineas deben ser agregadas o eliminadas entre ellos. Para lograr esto,
necesitamos identificar las lineas que son comunes entre ambos
archivos. Esto se enmarca en lo que se conoce como un problema LCS:
tenemos las dos secuencias de líneas y queremos encontrar la mayor
subsecuencia de lineas que aparecen en ambos archivos; estas líneas son
las que no fueron modificadas y las otras líneas son las que fueron
agregadas o eliminadas.
