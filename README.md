
# 🌌 Procedural Planet Renderer (Rust + Raylib)

Este proyecto es un **renderizador procedural de planetas y cuerpos astronómicos** escrito en **Rust**, utilizando la biblioteca **Raylib** para la visualización en tiempo real.

Permite cargar objetos `.obj`, aplicar transformaciones (traslación, rotación, escala) y generar materiales **procedurales** con múltiples capas de shaders (rocoso, gaseoso, anillos, lunares, etc.) para simular apariencias planetarias y efectos visuales dinámicos.

---

## 🚀 Características principales

- Sistema de cámara libre con controles para **rotar, orbitar y hacer zoom**.
- Entidades independientes (planetas, lunas, anillos) con sus propios **movimientos orbitales y de rotación**.
- **Shader procedural de 4 capas**:
  1. **L1 – Albedo / Paleta base:** define el color predominante (rocoso, gaseoso, etc.).
  2. **L2 – Bandas / Estrías:** patrones concéntricos, rayas o cuadros dinámicos.
  3. **L3 – Ruido / Perturbación:** texturas pseudoaleatorias generadas por ruido FBM.
  4. **L4 – Scanlines / Brillo:** efectos de resplandor, bloom y líneas tipo CRT.
- Sistema de **materiales personalizados por entidad** (`Material::rocky()`, `Material::gaseous()`, `Material::ring()`, `Material::star()`, `Material::moon()`).
- Control dinámico de shaders por teclado o mediante interfaz de botones.

---

## 🧠 Controles del programa

| Tecla | Acción |
|-------|--------|
| `W` / `S` | Inclinar cámara arriba / abajo |
| `A` / `D` | Girar cámara izquierda / derecha |
| `↑` / `↓` | Zoom in / out |
| `Q` / `E` | Paneo horizontal |
| `R` / `F` | Paneo vertical |
| `1–9` | Seleccionar entidad |
| `T` | Activar / desactivar shader de la entidad seleccionada |
| `G` | Alternar capa **L1 (albedo)** |
| `H` | Alternar capa **L2 (bandas)** |
| `J` | Alternar capa **L3 (ruido)** |
| `K` | Alternar capa **L4 (brillo)** |
| `Y` | Ver sólo el shader de la entidad seleccionada |
| `U` | Volver a vista de todos los shaders |

---

## 🪐 Materiales procedurales

Cada entidad tiene un material con parámetros que definen su estilo visual. Por ejemplo:

```rust
let planeta_rocoso = Material::rocky();
let planeta_gaseoso = Material::gaseous();
let anillo = Material::ring();
let luna = Material::moon();
let estrella = Material::star();
```

Puedes personalizar cada material modificando los campos del struct `Material`, como:
```rust
Material {
    pal_mix_radius: 0.7,
    rings_freq: 12.0,
    fbm_octaves: 5,
    bloom_strength: 0.1,
    ..Material::rocky()
}
```

---

## 🖼️ Capturas de pantalla

### Vista general del sistema planetario
![Sistema completo](./docs/screenshots/planets_overview.png)

---

## ⚙️ Compilación y ejecución

1. Asegúrate de tener instalado **Rust** y **Cargo**:
   ```bash
   rustup update
   ```
2. Clona el repositorio:
   ```bash
   git clone https://github.com/tu_usuario/wireframe
   cd wireframe
   ```
3. Compila y ejecuta:
   ```bash
   cargo run .
   ```

El programa abrirá una ventana de Raylib mostrando la escena 3D con los planetas generados.

---

## 🧩 Estructura del proyecto

```
wireframe/
├── src/
│   ├── main.rs
│   ├── framebuffer.rs
│   ├── camera.rs
│   ├── obj.rs
│   ├── matrix.rs
│   ├── line.rs
│   ├── triangle.rs
│   ├── fragment.rs
│   ├── shaders.rs
│   ├── procedural.rs
│   └── uniforms.rs
├── Cargo.toml
└── README.md
```

---

## 🧭 Créditos

- Renderizado en Rust con [raylib-rs](https://github.com/deltaphc/raylib-rs)
- Desarrollado como parte del laboratorio de **Gráficos por Computadora 2025**