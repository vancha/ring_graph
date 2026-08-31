use iced::advanced::graphics::geometry::frame::Backend;
use iced::advanced::layout::{Limits, Node};
use iced::advanced::{renderer, widget::Tree, Widget};
use iced::widget::{canvas, column, text, Canvas, Column};
use iced::{border, Center, Color, Element, Length, Point, Size};

//Alright, this is the struct that's eventually going to be the widget.. i'll try to just get it to
//draw itself first
struct RingGraph {
    radius: f32,
    values: Vec<i32>,
}

//This is the required trait for something that wants to present itself as an iced widget.
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for RingGraph
where
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    //This is required, just tell the caller that i want all the space please
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    //This layout::Node is used by the runtime to compute the Layout of the user interface. Looks
    //like limits is what we have available to us, lets turn that into a node that encapsulates it
    //all? Hopefully this will let us continue
    fn layout(&mut self, _: &mut Tree, _: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.max())
    }
    //Okay so apparently i'm just given a renderer and I should do something with this? First part
    //where i seem to be stuck.. lets investigate into what a renderer is even supposed to do..
    fn draw(
        &self,
        //
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        //the example thingy here can only do the fill_quad shape with a color and some borders...
        //can't use that for a cool ring graph :(
        let mut frame = renderer.new_frame(layout.bounds());
        //this is a color, i needed it for the line below
        let c_fil = Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };

        //I want to draw the ring graph at the center of the available space
        let frame_center = layout.bounds().center();


        let segments = self.draw_segments_around(frame_center);


        //and this path is the thing i think i actually eventually need... this is what i need
        //to get to work, cause it can work with arcs and stuff
        let path = iced::advanced::graphics::geometry::path::Path::new(
            move |path_builder: &mut iced::advanced::graphics::geometry::path::Builder| {
                let arc = iced::widget::canvas::path::arc::Arc {
                    center: frame_center,
                    radius: self.radius,
                    start_angle: iced::Radians(0.0),
                    end_angle: iced::Radians(4.0),
                };

                path_builder.arc(arc);
            },
        );
        let stroke = iced::advanced::graphics::geometry::stroke::Stroke::default().with_width(40.0).with_color(Color {r:0.5, g:0.2,b:0.8,a:1.0}); //iced::advanced::graphics::geometry::stroke::Stroke::with_width( 1.0);
        frame.stroke(&path, stroke);
        let geometry = frame.into_geometry();
        renderer.draw_geometry(geometry);
    }
}

//This contains the rest of the methods(?) that i want to add to the ring graph. I can't put it in the
//Widget trait implementation above, because my own functions are "Not a member of trait `Widget`"
impl RingGraph {
    //lets give ourselves an easy way to construct it? I don't know, this seems common in rust
    fn new(radius: f32, values: Vec<i32>) -> Self {
        RingGraph { radius, values }
    }

    //This should basically sum up all the values passed in as an argument, and
    //calculate what percentage(angle) of 100% (360 degrees) each individual
    //value has. Returns a vec of angles in degrees (or radians haven't figured that out)
    fn values_into_angles(&self, values: &Vec<i32>) -> Vec<f64> {
        let sum:i32 = values.iter().sum();
        //I think this is where i calculate what percentage every
        //individual value is of the total
        let percentages = values.into_iter().map(|value| (*value as f64 / sum as f64)*100.0 ).collect::<Vec<f64>>();
        println!("percentages: {:?}",percentages);
        let angles = percentages.into_iter().map(|percentage| 360.0 * (percentage / 100.0) ).collect::<Vec<f64>>();
        println!("angles: {:?}",angles);
        return angles;
    }

    //This is the thing that draws the individual segments using a renderer, which probably needs
    fn draw_segments_around(&self, center: Point) -> Vec<iced::advanced::graphics::geometry::path::Path> {
        let segments = self.values_into_angles(&self.values);

        segment.map( |angle|  )
        let path = iced::advanced::graphics::geometry::path::Path::new(
            move |path_builder: &mut iced::advanced::graphics::geometry::path::Builder| {
                let arc = iced::widget::canvas::path::arc::Arc {
                    center: center,
                    radius: self.radius,
                    start_angle: iced::Radians(0.0),
                    end_angle: iced::Radians(4.0),
                };

                path_builder.arc(arc);
            },
        );

        //vec![]
    }
}

//this weird looking thing makes the column![RingGraph::new(5.0), ] stuff possible
//you can't do that unless you can turn a RingGraph into an Element
impl<Message, Theme, Renderer> From<RingGraph> for Element<'_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::graphics::geometry::Renderer, //renderer::Renderer,
{
    fn from(ring_graph: RingGraph) -> Self {
        Self::new(ring_graph)
    }
}

//no message variants just yet, lets keep it empty
#[derive(Debug, Clone, Copy)]
enum Message {}

//The thing for which we implement our elm architecture functions
#[derive(Default)]
struct Application {}

impl Application {
    //Here we respond to updates that get emitted from the view, it should update our app state
    fn update(&mut self, message: Message) {
        match message {}
    }

    //Here we build our view from widgets and stuff
    fn view(&self) -> Column<'_, Message> {
        column![
            RingGraph::new(
                50.0,
                vec![1,2,3,4],
            )
        ]
    }
}

pub fn main() -> iced::Result {
    iced::run(Application::update, Application::view)
}
