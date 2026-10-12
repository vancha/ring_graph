//! This example showcases a simple native custom widget that draws a circle.
mod circle {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget, tree::{State,Tag}};
    use iced::advanced::graphics::geometry::path::Builder as PathBuilder;
    use iced::widget::{canvas};
    use iced::border;
    use iced::mouse;
    use iced::advanced::graphics::geometry::path::Path;
    //use iced::advanced::graphics::geometry::Cache;
    use std::ops::Add;
    use iced::{Color, Length, Rectangle, Size, Point};
    use iced::advanced::graphics::geometry::frame::Backend;
    
#[derive(Debug)]
struct Slice {
    path: iced::advanced::graphics::geometry::path::Path,
    color: Color,
}


    pub struct RingGraph {
        radius: f32,
        values: Vec<i32>,
    }

    pub struct RingGraphState {
        //cache: Cache<Renderer>,
        values: Vec<i32>,
    }

    impl RingGraphState  {
         fn new(values: Vec<i32>) -> Self {
            Self { values }
        }

    }

    impl RingGraph {

        pub fn new(radius: f32, values: Vec<i32>)  -> Self  {

            Self { radius: 100.0, values }
        }

        fn values_into_angles(&self, values: &Vec<i32>) -> Vec<f64> {
            let sum: i32 = values.iter().sum();
            //I think this is where i calculate what percentage every
            //individual value is of the total
            values
                .iter()
                .map(|value| (*value as f64 / sum as f64) * 100.0)
                .map(|percentage| 360.0 * (percentage / 100.0))
                .collect::<Vec<f64>>()
        }

        fn generate_slices_around(&self, center: Point) -> Vec<Slice> {
            let segments = self.values_into_angles(&self.values);
            let mut start_angle = 0.0f64;
            let mut paths = vec![];
            for segment in segments {
                let path = iced::advanced::graphics::geometry::path::Path::new(
                    |path_builder: &mut PathBuilder| {
                        let arc = canvas::path::arc::Arc {
                            center,
                            radius: self.radius,
                            start_angle: iced::Radians::from(iced::Degrees(start_angle as f32)),
                            end_angle: iced::Radians::from(iced::Degrees(
                                start_angle as f32 + segment as f32,
                            )),
                        };
                        path_builder.arc(arc);
                    },
                );

                start_angle += segment as f64;
                let num: [f32; 3] = rand::random();
                let slice = Slice {
                    path,
                    color: Color {
                        r: num[0],
                        g: num[1],
                        b: num[2],
                        a: 1.0,
                    },
                };

                paths.push(slice);
            }
            paths
        }
    }

    pub fn circle(radius: f32) -> RingGraph {
        RingGraph::new(radius, vec![1,2,3,4])
    }

    impl widget::Meta for RingGraph {}

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for RingGraph
    where
        Renderer: iced::advanced::graphics::geometry::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size::new(Length::Fill, Length::Fill)
        }

        fn layout(
            &mut self,
            tree: &mut widget::Tree,
            _renderer: &Renderer,
            limits: &layout::Limits,
        ) {
            //Node::new(limits.max())
            tree.size = limits.bounds();//Size::new(self.radius * 2.0, self.radius * 2.0);
        }

        fn draw(
            &self,
            _tree: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
        //the example thingy here can only do the fill_quad shape with a color and some borders...
        //can't use that for a cool ring graph :(
        let mut frame = renderer.new_frame(layout.bounds());
        //I want to draw the ring graph at the center of the available space
        let frame_center = layout.bounds().center();

        let slices = self.generate_slices_around(frame_center);

        for slice in slices {
            let stroke = iced::advanced::graphics::geometry::stroke::Stroke::default()
                .with_width(30.0)
                .with_color(slice.color)
                .with_line_cap(iced::widget::canvas::LineCap::Round); //iced::advanced::graphics::geometry::stroke::Stroke::with_width( 1.0);
            frame.stroke(&slice.path, stroke);
        }
        let geometry = frame.into_geometry();
        renderer.draw_geometry(geometry);
        }
        /* We need to have *something* that */
        fn state(&self) -> State { 
                State::Some(Box::new(RingGraphState::new(self.values.clone().into())))

         }
         fn tag(&self) -> Tag {
            Tag::of::<RingGraphState>()
         }
    }
}

use circle::circle;
use iced::widget::{center, column, slider, text};
use iced::{Center, Fit, Widget};

pub fn main() -> iced::Result {
    iced::run(Example::update, Example::view)
}

struct Example {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Example {
    fn new() -> Self {
        Example { radius: 50.0 }
    }

    fn update(&mut self, message: Message) {
    }

    fn view(&self) -> impl Widget<Message> {
        let content = column![
            circle(self.radius),
        ]
        .padding(20);
        //.spacing(20)
        //.width(Fit.max(500))
        //.align_x(Center);

        center(content)
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}

