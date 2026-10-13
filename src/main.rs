//! This example showcases a simple native custom widget that draws a circle.
mod ring_graph {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget, tree::{State,Tag}};
    use iced::advanced::graphics::geometry::path::Builder as PathBuilder;
    use iced::widget::{canvas};
    use iced::mouse;
    use iced::{Color, Length, Rectangle, Size, Point};
    use iced::advanced::graphics::geometry::frame::Backend;
    
    #[derive(Debug)]
    struct Slice {
        path: iced::advanced::graphics::geometry::path::Path,
        color: Color,
    }

    #[derive(Debug, Clone)]
    struct ColoredAngle {
        angle: f64,
        color: Color,
    }


    pub struct RingGraph {
        radius: f32,
        values: Vec<i32>,
    }

    #[derive(Debug)]
    pub struct RingGraphState {
        slices: Vec<ColoredAngle>
    }

    impl RingGraphState  {
         fn new( slices: Vec<ColoredAngle>) -> Self {
            Self {  slices}
        }

    }

    impl RingGraph {

        pub fn new(values: Vec<i32>)  -> Self  {

            Self { radius: 100.0, values }
        }
        //We can call this the very first time we get the values for our widget
        //that lets us remember it so we give it random colors only once, rather than every
        //draw call...
        fn values_into_angles(&self,  values: &[i32]) -> Vec<f64> {
            let sum: i32 = values.iter().sum();
            values
                .iter()
                .map(|value| (*value as f64 / sum as f64) * 100.0)
                .map(|percentage| 360.0 * (percentage / 100.0))
                .collect::<Vec<f64>>()
        }

        fn get_random_colors(&self, amount: usize) -> Vec<Color> {
            (0..amount).map(|_| {
                let num: [f32; 3] = rand::random();
                Color {
                        r: num[0],
                        g: num[1],
                        b: num[2],
                        a: 1.0,
                }
            }).collect()
        }

        //this could be what can actually be stored..
        fn get_initial_slices(&self) -> Vec<ColoredAngle> {
            let angles = self.values_into_angles(&self.values);
            let colors = self.get_random_colors(angles.len());
            colors.iter().zip(angles).map(|(a,b)|(*a,b)).map(|(c,d)| ColoredAngle { color: c, angle: d }).collect::<Vec<ColoredAngle>>()
        }

        fn generate_slices_around(&self, center: Point, angles: Vec<ColoredAngle>) -> Vec<Slice> {
            //let slices
            let segments = angles;//self.values_into_angles(&self.values);
            let mut start_angle = 0.0f64;
            let mut paths = vec![];
            for segment in segments {
                let path = iced::advanced::graphics::geometry::path::Path::new(
                    |path_builder: &mut PathBuilder| {
                        let arc = canvas::path::arc::Arc {
                            center,
                            radius: self.radius,
                            start_angle: iced::Radians::from(iced::Degrees(start_angle as f32)) - iced::Radians(0.01),
                            end_angle: iced::Radians::from(iced::Degrees((start_angle  + segment.angle) as f32)),
                        };
                        path_builder.arc(arc);
                    },
                );

                start_angle += segment.angle;
                paths.push(Slice {path,color: segment.color });
            }
            paths
        }
    }

    pub fn ring_graph(values: Vec<i32>) -> RingGraph {
        RingGraph::new( values )
    }

    impl widget::Meta for RingGraph {}

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for RingGraph
    where
        Renderer: iced::advanced::graphics::geometry::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size::new(Length::Fill, Length::Fill)
        }

        /* okay this seems to be the first time in the lifecycle of the program that i know the limits of where
        to draw myself.. 
        */
        fn layout(
            &mut self,
            tree: &mut widget::Tree,
            _renderer: &Renderer,
            limits: &layout::Limits,
        ) {
            //tried to get the sate from the tree.. doesn't work without &&mut... 
            //let my_state: &&mut RingGraphState = &tree.state.downcast_mut::<RingGraphState>();
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
        let staty_staty_boi = _tree.state.downcast_ref::<RingGraphState>();
        let slices = self.generate_slices_around(frame_center, staty_staty_boi.slices.clone());

        for slice in slices {
            let stroke = iced::advanced::graphics::geometry::stroke::Stroke::default()
                .with_width(40.0)
                .with_color(slice.color);
                //.with_line_cap(iced::widget::canvas::LineCap::Round); //iced::advanced::graphics::geometry::stroke::Stroke::with_width( 1.0);
            frame.stroke(&slice.path, stroke);
        }
        let geometry = frame.into_geometry();
        renderer.draw_geometry(geometry);
        }
        /* this method is called exactly once! so that's useful so do some expensive work*/
        fn state(&self) -> State { 
                println!("CREATING LE STATE.. should happen exactly only once :)");
                State::Some(Box::new( RingGraphState::new(  self.get_initial_slices()) ))

         }
         /* This i don't really get, but it seems how other iced widgets are doing it */
         fn tag(&self) -> Tag {
            Tag::of::<RingGraphState>()
         }
    }
}

use ring_graph::ring_graph;
use iced::widget::{center, column };
use iced::Widget;

pub fn main() -> iced::Result {
    iced::run(Example::update, Example::view)
}

struct Example {
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Example {
    fn new() -> Self {
        Example {  }
    }

    fn update(&mut self, _message: Message) {
    }

    fn view(&self) -> impl Widget<Message> {
        let content = column![
            ring_graph(vec![1,2,4, 66,3,4,55]),
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

