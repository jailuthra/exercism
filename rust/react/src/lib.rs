use std::collections::HashMap;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(u32);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct ComputeCell<'a, T> {
    deps: Vec<CellID>,
    func: fn(&[T]) -> T,
    callbacks: HashMap<u32, Box<dyn FnMut(T) + 'a>>,
    counter: u32,
}

pub struct Reactor<'a, T> {
    cells: HashMap<u32, T>,
    compute_cells: HashMap<u32, ComputeCell<'a, T>>,
    counter: u32,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {cells: HashMap::new(), compute_cells: HashMap::new(), counter: 0}
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.counter += 1;
        self.cells.insert(self.counter, initial);
        InputCellID(self.counter)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute(
        &mut self,
        dependencies: &[CellID],
        compute_func: fn(&[T]) -> T,
    ) -> Result<ComputeCellID, CellID> {
        let compute_cell = ComputeCell{deps: dependencies.to_vec(), func: compute_func, callbacks: HashMap::new(), counter: 0};
        let initial = self.cell_compute_val(&compute_cell);
        match initial {
            Ok(initial) => {
                self.counter += 1;
                self.compute_cells.insert(self.counter, compute_cell);
                self.cells.insert(self.counter, initial);
                Ok(ComputeCellID(self.counter))
            },
            Err(dep) => Err(dep)
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(id)) => self.cells.get(&id).copied(),
            CellID::Compute(ComputeCellID(id)) => self.cells.get(&id).copied(),
        }
    }

    // Compute result of a compute-cell given the current values of its dependencies
    fn cell_compute_val(&self, cell: &ComputeCell<T>) -> Result<T, CellID> {
        let mut dependency_values = Vec::new();
        for &dep in &cell.deps {
            let val = match dep {
                CellID::Compute(ComputeCellID(id)) => self.cells.get(&id),
                CellID::Input(InputCellID(id)) => self.cells.get(&id)
            };
            match val {
                Some(&cell_data) => dependency_values.push(cell_data),
                None => return Err(dep) 
            };
        }
        let initial = (cell.func)(&dependency_values);
        Ok(initial)
    }

    // Propogate values until all compute-cells are unchanged
    // Fire callbacks for changed values
    fn propagate_and_fire_callbacks(&mut self) {
        let old_vals = self.cells.clone();
        loop {
            let mut changed = false;
            for (id, compute_cell) in &self.compute_cells {
                if let Ok(val) = self.cell_compute_val(compute_cell) { 
                    if let Some(&old_val) = self.cells.get(id) {
                        if val != old_val {
                            // Values changed, update our hashmap
                            self.cells.insert(*id, val);
                            changed = true;
                        }
                    }
                }
            }
            if !changed { break; }
        }
        // fire callbacks for updated cells
        for ((id, &old_val),(_, &new_val)) in old_vals.iter().zip(self.cells.iter()) {
            if old_val != new_val {
                if let Some(compute_cell) = self.compute_cells.get_mut(&id) {
                    for (_, callback) in compute_cell.callbacks.iter_mut() {
                        callback(new_val);
                    }
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if self.cells.contains_key(&id.0) {
            self.cells.insert(id.0, new_value);
            self.propagate_and_fire_callbacks();
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if let Some(compute_cell) = self.compute_cells.get_mut(&id.0) {
            compute_cell.counter += 1;
            compute_cell.callbacks.insert(compute_cell.counter, Box::new(callback));
            Some(CallbackID(compute_cell.counter))
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(compute_cell) = self.compute_cells.get_mut(&cell.0) {
            match compute_cell.callbacks.remove(&callback.0) {
                None => Err(RemoveCallbackError::NonexistentCallback),
                _ => Ok(()),
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
