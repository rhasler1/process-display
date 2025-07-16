### Overall Design

/components:    UI rendering                        --      how state is drawn & responds to events
/state:         UI state                            --      logical state that drives the UI
/models:        Domain data & business logic        --      set of rules, computations, and workflows that define how the app solves real-world problems

/state:
    ListState: selection, sort<S>, vec<T>, filter

/models:
    ProcessItem
    ProcessList

### Design
- state/: contains UI state, not rendering code. 

#### UI


----

#### Backend
1. Backend should provide indices to UI views
   1. E.g., UI calls for sorted representation of ProcessList, Backend should return a Vec<usize> where each element referes to an index in the original Vec<ProcessItem>
      1. Idea: Implement iterator "functions?" that created the correct references vectors
      2. E.g., Vec<u64> = [1, 3, 4, 2]; UI calls to sort, backend returns: Vec<usize> = [0, 3, 1, 2];
      3. the Vec<T> in ListState should be immutable (Not mut methods allowed)
      4. Idea: It might be worth storing a reference to the selected item in Vec<T> 

ListState: Vec<T>