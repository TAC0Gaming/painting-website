use macroquad::prelude::*;

fn window_config() -> Conf {
    return Conf {
        window_title: "Paint".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Copy, Clone)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
    color: Color
}

struct Button {
    x: i32,
    y: i32,
    text: String,
    text_size: u16,
    padding: i32,
    text_dimensions: TextDimensions,
    hover: bool
}

impl Button {
    fn new(x: i32, y: i32, label: String, text_size: u16, padding: i32) -> Self {
        return Self {
            x: x,
            y: y,
            text: label.clone(),
            text_size: text_size,
            padding: padding,
            text_dimensions: measure_text(&label, None, text_size, 1.0),
            hover: false
        };
    }

    fn draw(&self) {
        
        let text_size = self.text_dimensions;

        let color = match self.hover {
            false => WHITE,
            true => Color { r: 0.875, g: 0.875, b: 0.875, a: 1.0 }
        };

        draw_rectangle_lines(self.x as f32-self.padding as f32-text_size.width/2.0, self.y as f32 - text_size.height/2.0 as f32-self.padding as f32, text_size.width+self.padding as f32*2.0, text_size.height+self.padding as f32*2.0, 5.0, color);
        draw_text(&self.text, self.x as f32 - text_size.width/2.0, self.y as f32 + text_size.height/2.0, self.text_size as f32, color);

    }

    fn is_over(&self, x: i32, y: i32) -> bool {
        if x > self.x - self.padding - self.text_dimensions.width as i32/2 && x < self.x + self.padding + self.text_dimensions.width as i32/2 {
            if y > self.y - self.padding - self.text_dimensions.height as i32/2 && y < self.y + self.padding + self.text_dimensions.height as i32/2 {
                return true;
            }
        }
        return false;
    }

}

#[macroquad::main(window_config)]
async fn main() {
    let mut canvas: Vec<Circle> = Vec::new();

    let mut current_color: Color = WHITE;

    let mut red_button = Button::new(70, screen_height() as i32 - 30, "RED".to_string(), 90, 5);

    let mut orange_button = Button::new(260, screen_height() as i32 - 30, "ORANGE".to_string(), 90, 5);

    let mut yellow_button = Button::new(510, screen_height() as i32 - 30, "YELLOW".to_string(), 90, 5);

    let mut green_button = Button::new(740, screen_height() as i32 - 30, "GREEN".to_string(), 90, 5);

    let mut blue_button = Button::new(930, screen_height() as i32 - 30, "BLUE".to_string(), 90, 5);

    let mut purple_button = Button::new(1140, screen_height() as i32 - 30, "PURPLE".to_string(), 90, 5);

    let mut white_button = Button::new(1370, screen_height() as i32 - 30, "WHITE".to_string(), 90, 5);

    let mut eraser_button = Button::new(1580, screen_height() as i32 - 30, "ERASE".to_string(), 90, 5);

    let mut radius = 10.0;

    loop {
        let pos = mouse_position();
        draw_circle(pos.0, pos.1, 10.0, Color { r: current_color.r, g: current_color.g, b: current_color.b, a: 0.45 });
        if is_mouse_button_down(MouseButton::Left){
            canvas.push(Circle { x: pos.0, y: pos.1, r: radius, color: current_color});
            if red_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = RED;
                radius = 10.0; //get rid of radius when you add radius changer button
            }
            if orange_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = ORANGE;
                radius = 10.0; //get rid of radius when you add radius changer button
            }
            if yellow_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = YELLOW;
                radius = 10.0; //get rid of radius when you add radius changer button
            }
            if green_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = GREEN;
                radius = 10.0; //get rid of radius when you add radius changer button
            }
            if blue_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = BLUE;
                radius = 10.0; //get rid of radius when you add radius changer button
            }
            if purple_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = PURPLE;
                radius = 10.0; //get rid of radius when you add radius changer button
            }
            if white_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = WHITE;
                radius = 10.0; //get rid of radius when you add radius changer button
            }
            if eraser_button.is_over(pos.0 as i32,pos.1 as i32){
                current_color = BLACK;
                radius = 20.0; //get rid of radius when you add radius changer button
            }

        }
        for i in &canvas {
            draw_circle(i.x, i.y, i.r, i.color);
        }
        
        red_button.draw();
        orange_button.draw();
        yellow_button.draw();
        green_button.draw();
        blue_button.draw();
        purple_button.draw();
        white_button.draw();
        eraser_button.draw();

        next_frame().await
    }
}
