use enigo::{Coordinate, Enigo, Mouse, Settings};
use rand::Rng;
use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut rng = rand::thread_rng();

    // Получаем разрешение экрана
    let screen_width = 800;
    let screen_height = 600;

    // Начальные координаты курсора
    let mut current_x = screen_width / 2;
    let mut current_y = screen_height / 2;

    loop {
        // Генерируем случайные координаты
        let target_x = rng.gen_range(0..screen_width);
        let target_y = rng.gen_range(0..screen_height);

        // Плавное перемещение курсора
        let steps = 100; // Количество шагов для плавного перемещения
        for i in 0..steps {
            // Интерполяция между текущими и целевыми координатами
            current_x = current_x + (target_x - current_x) / (steps - i);
            current_y = current_y + (target_y - current_y) / (steps - i);

            // Перемещаем курсор
            let _ = enigo.move_mouse(current_x, current_y, Coordinate::Abs);

            // Задержка для создания эффекта плавного движения
            let delay = time::Duration::from_millis(10); // 10 миллисекунд
            thread::sleep(delay);
        }
    }
}
