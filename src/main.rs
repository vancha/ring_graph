use iced::advanced::layout::{Limits, Node};
use iced::advanced::{renderer, widget::Tree, Widget};
use iced::widget::{canvas, column, text, Canvas, Column};
use iced::{border, Center, Color, Element, Point, Size, Length};
use iced::advanced::graphics::geometry::frame::Backend;
//use iced::advanced::iced_graphics::geometry::frame::Backend

//Alright, this is the struct that's eventually going to be the widget.. i'll try to just get it to
//draw itself first
struct RingGraph {
    radius: f32,
}

//This is the required trait for something that wants to present itself as an iced widget.
impl<Message, Theme, Renderer> iced::advanced::Widget<Message, Theme, Renderer> for RingGraph
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
        let c_fil = Color {r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
        //apparetnly, we can draw a rectangle! and the final argument is anything htat can be turend
        //in to a "fill", that includes colors
        //frame.fill_rectangle(Point::new(10.0, 10.0), Size::new(10.0, 10.0),  c_fil );

        //and this path is the thing i think i actually eventually need... this is what i need
        //to get to work, cause it can work with arcs and stuff
        let path = iced::advanced::graphics::geometry::path::Path::new( move |build_thingy: &mut iced::advanced::graphics::geometry::path::Builder| {
            build_thingy.move_to(Point::new(0.0, 0.0)); 
            build_thingy.line_to(Point::new(100.0, 100.0));
            build_thingy.move_to(Point::new(100.0, 450.0)); 
            //build_thingy.arc_to(Point::new(100.0, 200.0), Point::new(10.0, 100.0),  180.0);
        }
        );
        let stroke = iced::advanced::graphics::geometry::stroke::Stroke::default(); //iced::advanced::graphics::geometry::stroke::Stroke::with_width( 1.0);
        frame.stroke(&path,stroke );
        let geometry = frame.into_geometry();
        renderer.draw_geometry(geometry);
        
    }
}

//lets give ourselves an easy way to construct it? I don't know, this seems common, i've seen other peopel do it
impl RingGraph {
    fn new(radius: f32) -> Self {
        RingGraph { radius }
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
        column![RingGraph::new(50.0),]
    }
}

pub fn main() -> iced::Result {
    iced::run(Application::update, Application::view)
}
