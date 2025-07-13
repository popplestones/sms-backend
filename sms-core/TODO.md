- [x] Add schedule_at Option<DateTime<Utc>> and delay_ms: Option<u64> to OutgoingJob.
- [ ] Add helper method on outgoing job:
```rust
impl OutgoingJob
    pub fn scheduled_timestamp(&self) -> Option<i64> { ... }
}
```
- [ ] Add validation methods (e.g. is_valid(), is_scheduled_in_past())
