//! This example showcases a simple native custom widget that draws a circle.
mod ring_graph {
    use iced::advanced::graphics::geometry::fill::Fill;
    use iced::advanced::graphics::geometry::frame::Backend;
    use iced::advanced::graphics::geometry::path::Builder as PathBuilder;
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{
        self,
        tree::{State, Tag},
        Widget,
    };
    use iced::mouse;
    use iced::widget::canvas;
    use iced::{Color, Length, Point, Rectangle, Size};
    use std::cmp::min;

    //For some reason when turning arcs into strokes, there's always a little cap visible in the eventual
    //ring chart. Just adding a small offset makes that disappear.
    const GAP_CLOSING_OFFSET: iced::Radians = iced::Radians(0.01);
    const STROKE_WIDTH: f32 = 40.0;

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
        values: Vec<i32>,
    }

    #[derive(Debug)]
    pub struct RingGraphState {
        slices: Vec<ColoredAngle>,
    }

    impl RingGraphState {
        fn new(slices: Vec<ColoredAngle>) -> Self {
            Self { slices }
        }
    }

    impl RingGraph {
        pub fn new(values: Vec<(String, i32)>) -> Self {
            Self {
                //radius: 100.0,
                values: values.into_iter().map(|(a, b)| b).collect::<Vec<_>>(),
            }
        }
        //We can call this the very first time we get the values for our widget
        //that lets us remember it so we give it random colors only once, rather than every
        //draw call...
        fn values_into_angles(&self, values: &[i32]) -> Vec<f64> {
            let sum: i32 = values.iter().sum();
            values
                .iter()
                .map(|value| (*value as f64 / sum as f64) * 100.0)
                .map(|percentage| 360.0 * (percentage / 100.0))
                .collect::<Vec<f64>>()
        }

        fn get_random_colors(&self, amount: usize) -> Vec<Color> {
            (0..amount)
                .map(|_| {
                    let num: [f32; 3] = rand::random();
                    Color {
                        r: num[0],
                        g: num[1],
                        b: num[2],
                        a: 1.0,
                    }
                })
                .collect()
        }

        //this could be what can actually be stored..
        fn get_initial_slices(&self) -> Vec<ColoredAngle> {
            let angles = self.values_into_angles(&self.values);
            let colors = self.get_random_colors(angles.len());
            colors
                .iter()
                .zip(angles)
                .map(|(c, d)| ColoredAngle {
                    color: *c,
                    angle: d,
                })
                .collect::<Vec<ColoredAngle>>()
        }
        //this now no longer does any color generation, it just takes the angles and colors as an argument
        fn generate_slices_around(
            &self,
            angles: Vec<ColoredAngle>,
            available_space: iced::Rectangle,
        ) -> Vec<Slice> {
            println!("space: {:?}", available_space);
            //okay lets see how we want to calculate the size of our little ring.
            //we want to know the smallest out of both width and height, because we need to assume we're drawing inside a
            //little square, not a rectangle.
            //so maybe let size = min()
            //also make sure that we can't let the radius of our circle go past the edge of the space
            //and ignore the width of the stroke itself

            let segments = angles;
            let mut start_angle = 0.0f64;
            let mut paths = vec![];
            let width = match available_space.width < available_space.height {
                true => available_space.width,
                false => available_space.height,
            } - STROKE_WIDTH;
            for segment in segments {
                let path = iced::advanced::graphics::geometry::path::Path::new(
                    |path_builder: &mut PathBuilder| {
                        let running_start = iced::Radians::from(iced::Degrees(start_angle as f32))
                            - GAP_CLOSING_OFFSET;
                        let running_end = iced::Radians::from(iced::Degrees(
                            (start_angle + segment.angle) as f32,
                        ));
                        let arc = canvas::path::arc::Arc {
                            center: available_space.center(),
                            radius: width / 2.0,
                            start_angle: running_start,
                            end_angle: running_end,
                        };
                        path_builder.arc(arc);
                    },
                );

                start_angle += segment.angle;
                paths.push(Slice {
                    path,
                    color: segment.color,
                });
            }
            paths
        }
    }

    pub fn ring_graph(values: Vec<(String, i32)>) -> RingGraph {
        RingGraph::new(values)
    }

    impl widget::Meta for RingGraph {}

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for RingGraph
    where
        Renderer: iced::advanced::graphics::geometry::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size::new(Length::Fill, Length::Fill)
        }

        /* this seems to be what i need to do things like detect if there's someone hovering their mouse
        over a graph segment.. Which in turn i will need to do to implement the animation i want. Idea:
        when segment is hovered over, decrease saturation of all other segments, and make highlighted
        segment increase in size. Have said animation take about 250ms or something? */
         fn update(&mut self, _tree: &mut widget::Tree, _event: &iced::Event, _layout: iced::advanced::Layout, _cursor: iced::mouse::Cursor, _renderer: &Renderer, _shell: &mut iced::advanced::Shell<'_, Message>, _viewport: &iced::Rectangle) {
            println!("I have the cursor! i have it! ;D {:?}",_cursor);
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

            let smaller = limits.bounds().width.min(limits.bounds().height);

            tree.size = Size {
                width: smaller,
                height: smaller,
            };
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
            let available_space = layout.bounds();
            let staty_staty_boi = _tree.state.downcast_ref::<RingGraphState>();
            let slices =
                self.generate_slices_around(staty_staty_boi.slices.clone(), available_space);

            for slice in slices {
                let stroke = iced::advanced::graphics::geometry::stroke::Stroke::default()
                    .with_width(STROKE_WIDTH)
                    .with_color(slice.color);
                //.with_line_cap(iced::widget::canvas::LineCap::Round); //iced::advanced::graphics::geometry::stroke::Stroke::with_width( 1.0);
                frame.stroke(&slice.path, stroke);
                //frame.fill_rectangle(Point::new(0.0,0.0),layout.bounds().into(), Fill::default());
            }
            let geometry = frame.into_geometry();

            renderer.draw_geometry(geometry);
        }

        /* this method is called exactly once! so that's useful so do some expensive work*/
        fn state(&self) -> State {
            println!("CREATING LE STATE.. should happen exactly only once :)");
            State::Some(Box::new(RingGraphState::new(self.get_initial_slices())))
        }
        /* This i don't really get, but it seems how other iced widgets are doing it */
        fn tag(&self) -> Tag {
            Tag::of::<RingGraphState>()
        }
    }
}

use iced::widget::{center, column};
use iced::Subscription;
use iced::Widget;
use ring_graph::ring_graph;
use iced::time::{self, Duration, Instant};


pub fn main() -> iced::Result {
    /* to be able to use an iced subscription, i can't seem to use iced::run, but i will have to change to
    iced::application. Then i can pass in a subscription. */
    //iced::run(Example::update, Example::view)
    iced::application(Example::default, Example::update, Example::view)
    //.subscription(Example::subscription)
    .run()
}

struct Example {
    timer_enabled:bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ExampleMessage(Instant)
}

impl Example {
    fn new() -> Self {
        Example {timer_enabled: true}
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ExampleMessage(i) => {
                println!("received an example message with instant: {:?}",i);
            }
        }
    }

    fn view(&self) -> impl Widget<Message> {
        /* widget takes owned values at the moment for simplicity. I can imagine other people will want to plot
        data that is part of their applications struct for real world usecases.. that'll be implemented later*/
        let content = column![ring_graph(vec![
            (String::from("one"), 1),
            (String::from("two"), 2),
            (String::from("three"), 3),
            (String::from("four"), 4),
            (String::from("five"), 5)
        ])]
        .padding(25);
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
