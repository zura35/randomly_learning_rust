use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum OrderStatus {
    Created,
    Updated,
    Paid,
    Cancelled,
}

pub enum OrderEventType {
    Created{
        user_id: Uuid,
        amount: f64,
    },
    Updated{
        amount: f64,
    },
    Paid,
    Cancelled,
}

pub struct OrderEvent {
    pub id: Uuid,
    pub order_id: Uuid,
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub event_type: OrderEventType,
}

impl OrderEvent {
    fn apply(&self, aggregate: &mut OrderAggregate) {
        match self.event_type {
            OrderEventType::Created{user_id, amount} => {
                aggregate.id = self.order_id;
                aggregate.user_id = user_id;
                aggregate.amount = amount;
                aggregate.status = OrderStatus::Created;
            },
            OrderEventType::Updated{amount} => {
                if self.is_event_applicable(aggregate) == false {
                    panic!("ERR: Invalid event");
                }
                aggregate.amount = amount;
                aggregate.status = OrderStatus::Updated;
            },
            OrderEventType::Paid => {
                if self.is_event_applicable(aggregate) == false {
                    panic!("ERR: Invalid event");
                }
                aggregate.status = OrderStatus::Paid;
            },
            OrderEventType::Cancelled => {
                if self.is_event_applicable(aggregate) == false {
                    panic!("ERR: Invalid event");
                }
                aggregate.status = OrderStatus::Cancelled;
            },
        }

        aggregate.version = self.version;
        aggregate.created_at = self.created_at;
    }

    fn is_event_applicable(&self, aggregate: &OrderAggregate) -> bool {
        match self.event_type {
            OrderEventType::Created{..} => { true }, // nothing to check
            _ => {
                self.order_id == aggregate.id && self.version == aggregate.version + 1
            },
        }
    }
}

#[derive(Debug)]
pub struct OrderAggregate {
    id: Uuid,
    user_id: Uuid,
    version: u32,
    amount: f64,
    status: OrderStatus,
    created_at: DateTime<Utc>,
}

impl OrderAggregate {
    pub fn new() -> OrderAggregate {
        OrderAggregate {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            version: 0,
            amount: 0.0,
            status: OrderStatus::Created,
            created_at: Utc::now(),
        }
    }
}

pub struct OrderAggregateProjector;

impl OrderAggregateProjector {
    pub fn new() -> OrderAggregateProjector {
        OrderAggregateProjector {}
    }

    // set target_version to 0 to replay all events
    pub fn replay(
        &self,
        events: Vec<OrderEvent>,
        target_version: u32,
    ) -> OrderAggregate {
        if events.is_empty() {
            panic!("ERR: No events to replay");
        }

        let version = if target_version > 0 {
            target_version
        } else {
            let n = u32::try_from(events.len());
            n.unwrap()
        };

        let mut aggregate = OrderAggregate::new();
        for event in events {
            if event.version > version {
                break;
            }
            event.apply(&mut aggregate);
        }

        aggregate
    }
}