pub trait Command<Context, IntState, Error>
where
    Self: State<Context, State = IntState>
        + Check<Context, IntState, Error = Error>
        + Execute<Context, IntState, Error = Error>,
{
    fn process(&self, context: Context) -> Result<(), Error> {
        let state = self.state(&context);
        self.check(&context, &state)
            .and_then(|_| self.execute(&context, &state))
    }
}

pub trait State<Context> {
    type State;
    fn state(&self, context: &Context) -> Self::State;
}

pub trait Check<Context, State> {
    type Error;
    fn check(&self, context: &Context, state: &State) -> Result<(), Self::Error>;
}

pub trait Execute<Context, State> {
    type Error;
    fn execute(&self, context: &Context, state: &State) -> Result<(), Self::Error>;
}
