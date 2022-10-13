/*
    TOPIC: Typestates

    Summary:
    An airline wants to reduce the amount of lost luggage by ensuring luggage is properly tracked

    Requirements;
    - implement a luggage tracking system using the typestate pattern
    - Each piece of luggage has a tracking id
    - Luggage goes through multiple states at the airport:
        - check-in  (passenger gives luggage to airport)
        - OnLoading (luggage is loaded onto correct plane)
        - Offloading (luggage is taken off plane at destination)
        - AwaitingPickup (luggage is at destination waiting for passenger pickup)
        - EndCustody (luggage was picked up by passenger)
    NOTES:
    Optionally use generics for each state
*/
#[derive(Debug)]
struct Luggage<State> {
    id: u64,
    customer: String,
    state: State,
}
struct Checkin;
struct OnLoading;
struct OffLoading;
struct AwaitingPickup;
#[derive(Debug)]
struct EndCustody;

impl<State> Luggage<State> {
    fn transition<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage {
            id: self.id,
            customer: self.customer,
            state,
        }
    }
}

impl Luggage<Checkin> {
    fn new(id: u64, customer: String) -> Self {
        Self {
            id,
            customer,
            state: Checkin,
        }
    }

    fn checkin_luggage(self) -> Luggage<OnLoading> {
        self.transition(OnLoading)
    }
}

impl Luggage<OnLoading> {
    fn load(self) -> Luggage<OffLoading> {
        self.transition(OffLoading)
    }
}

impl Luggage<OffLoading> {
    fn offload(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}
impl Luggage<AwaitingPickup> {
    fn pick_up(self) -> Luggage<EndCustody> {
        self.transition(EndCustody)
    }
}


fn main() {
    let peters_luggage = Luggage::new(8383, "Peter Tyonum".to_owned());
    let boarded_luggage = peters_luggage.checkin_luggage().load().offload().pick_up();

    println!("Customer luggage: {:?}", boarded_luggage);
}
