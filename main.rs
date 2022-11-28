//extern crate eval;
use yew::prelude::*;
//use eval::eval;

enum Msg {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Point,
    Div,
    Mul,
    Plus,
    Sub,
    Left,
    Right,
    Equal,
    Backspace,
    Clr,
}


struct Calc {
    equation: String,
    //total: Result<Value, eval::Error>,
}

impl Component for Calc {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            equation: String::from(""),
            //total: Result<0>,

        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Zero => {
                self.equation.push('0');
                true
            }
            Msg::One => {
                self.equation.push('1');
                true
            }
            Msg::Two => {
                self.equation.push('2');
                true
            }
            Msg::Three => {
                self.equation.push('3');
                true
            }
            Msg::Four => {
                self.equation.push('4');
                true
            }
            Msg::Five => {
                self.equation.push('5');
                true
            }
            Msg::Six => {
                self.equation.push('6');
                true
            }
            Msg::Seven => {
                self.equation.push('7');
                true
            }
            Msg::Eight => {
                self.equation.push('8');
                true
            }
            Msg::Nine => {
                self.equation.push('9');
                true
            }
            Msg::Point => {
                self.equation.push('.');
                true
            }
            Msg::Div => {
                self.equation.push('/');
                true
            }
            Msg::Mul => {
                self.equation.push('*');
                true
            }
            Msg::Plus => {
                self.equation.push('+');
                true
            }
            Msg::Sub => {
                self.equation.push('-');
                true
            }
            Msg::Left => {
                self.equation.push('(');
                true
            }
            Msg::Right => {
                self.equation.push(')');
                true
            }
            Msg::Equal => {
                /*match eval::eval(&*self.equation) {
                    Ok(_value) => self.total = eval::eval(&*self.equation).unwrap().as_f64().unwrap(),
                    //self.equation = self.total.to_string(),
                    Err(_error) => ()
                    //self.equation.clear(),
                    //self.equation.push.("X"),
                }
                */
                //self.total = eval(&self.equation);
                self.equation.push('=');
                true
            }
            Msg::Backspace => {
                self.equation.pop();
                true
            }
            Msg::Clr => {
                self.equation.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
       let link = ctx.link();
        html! {
            <div class="container">
            <h1>{"Calculator"}</h1>
                <p>{ &self.equation }</p>
                //<p>{ &self.total }</p>
                <div class = "row1">
                <button onclick={link.callback(|_| Msg::Zero)}>{ "0" }</button>
                <button onclick={link.callback(|_| Msg::Clr)}>{ "C" }</button>
                <button onclick={link.callback(|_| Msg::Point)}>{ "." }</button>
                <button onclick={link.callback(|_| Msg::Div)}>{ "/" }</button><br/>
                </div>
                <div class = "row2">
                <button onclick={link.callback(|_| Msg::Seven)}>{ "7" }</button>
                <button onclick={link.callback(|_| Msg::Eight)}>{ "8" }</button>
                <button onclick={link.callback(|_| Msg::Nine)}>{ "9" }</button>
                <button onclick={link.callback(|_| Msg::Mul)}>{ "x" }</button><br/>
                </div>
                <div class = "row3">
                <button onclick={link.callback(|_| Msg::Four)}>{ "4" }</button>
                <button onclick={link.callback(|_| Msg::Five)}>{ "5" }</button>
                <button onclick={link.callback(|_| Msg::Six)}>{ "6" }</button>
                <button onclick={link.callback(|_| Msg::Sub)}>{ "-" }</button><br/>
                </div>
                <div class = "row4">
                <button onclick={link.callback(|_| Msg::One)}>{ "1" }</button>
                <button onclick={link.callback(|_| Msg::Two)}>{ "2" }</button>
                <button onclick={link.callback(|_| Msg::Three)}>{ "3" }</button>
                <button onclick={link.callback(|_| Msg::Plus)}>{ "+" }</button><br/>
                </div>
                <div class = "row5">
                <button onclick={link.callback(|_| Msg::Left)}>{ "(" }</button>
                <button onclick={link.callback(|_| Msg::Right)}>{ ")" }</button>
                <button onclick={link.callback(|_| Msg::Backspace)}>{ "<" }</button>
                <button onclick={link.callback(|_| Msg::Equal)}>{ "=" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Calc>();
}