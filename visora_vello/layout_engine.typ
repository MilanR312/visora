Container1
- Vlist
    - Text1
    - Container2
        - Text2
    

+ screen is 100x100
+ container 1 has insets 5
+ container 2 has insets 10
+ text1 is 10 high
+ text2 is 20 high
#set page("a3")


#table(
    columns: 6,
    inset: 0.5em,
    [Type], [parent_constraint], [remaining parent], [self constraint], [self_size], [operation],

    [Before], [none], [x], [(100, 100)], [], [init],
    
    [Container1 #sym.arrow.b], [Before: (100, 100)], [x], [(90,90)], [], [push(constraint)],
    
    [Vlist #sym.arrow.b], [C1: (90, 90)], [x], [], [], [],
    
    [Text1 #sym.arrow.b], [C1: (90, 90)], [x], [], [], [],

    [Text1 #sym.arrow.t], [C1: (90, 90)], [C1: (90, 90) ], [], [(90, 10)], [calc_size(text)],
    [], [C1: (90, 90)], [C2: (90, 80)], [], [], [remaining -= size],
    
    [Container2 #sym.arrow.b], [C1: (90, 90)], [C1: (90, 80)], [(70, 60)], [], [push_constraint(remaining -= padding)],
    
    [Text2 #sym.arrow.b], [C2: (70, 60)], [C2: (70, 60)], [], [], [],
    
    [Text2 #sym.arrow.t], [C2: (70, 60)], [C2: (70, 60)], [], [(70, 20)], [calc_size(text2)],
    [], [C2: (70, 60)], [C2: (70, 40)], [], [], [remaining -= size],

    [Container2 #sym.arrow.t], [C2: (70, 60)], [C2: (70, 40)], [], [(90, 40)], [calc_size(parent-remaining + padding)],
    [], [C1: (90, 90)], [C1: (90, 40)], [], [(90, 40)], [remaining -= size],
    //[], [C1: (90, 90)], [C1: (90, 40)], [], [(90, 50)], [calc_size(parent-remaining)],

    [Vlist #sym.arrow.t], [C1: (90, 90)], [C1: (90, 40)], [], [(90, 50)], [calc_size(constraint-remaining)],

    
    [Container1 #sym.arrow.t], [C1: (90, 90)], [C1: (90, 40)], [], [(100, 60)], [calc_size(parent-remaining + padding)],
    [], [Before: (100, 100)], [Before: (100, 40)], [], [(100, 60)], [remaining -= size],
    //[], [Before: (100, 100)], [Before: (100, 40)], [], [(100, 60)], [calc_size(parent-remaining)],
)

/*
#table(
    columns: 7,
    inset: 1em,
    [Type], [Direction], [Constraints], [new/popped constraint], [modified constraint], [sizes], [new size],
    [Before], [], [()], [()], [(100,100)], [], [],

    [Container1], [#sym.arrow.b], [(100,100)], [("nc")],[n: (90,90)], [], [],
    
    [Vlist], [#sym.arrow.b], [(100,100), (90,90)], [("nc"), ("nc")], [none], [], [],

    [Text1], [#sym.arrow.b], [(100,100), (90, 90)], [("nc"), ("nc")], [none], [], [],

    [Text1], [#sym.arrow.t], [(100, 100), (90, 90)], [("nc"), ("nc")], [none], [], [(90, 10)],


    [Container2], [#sym.arrow.b], [(100, 100), (90, 90)], [n: (70, 70)], [(90, 10)], [],
    [Text2], [#sym.arrow.b], [(100, 100), (90, 90), (70, 70)], [none], [(90,10)], [],
    [Text2], [#sym.arrow.t], [(100, 100), (90, 90), (70, 70)], [none], [(90, 10)], [(70, 20)],
    [Container2], [#sym.arrow.t], [(100, 100), (90, 90)], [p: (70, 70)], [(90, 10), (70, 20)], [(90, 40)],
    [Vlist], [#sym.arrow.t], [(100, 100), (90, 90)], [none], [(90, 10), (70, 20), (90, 40)], [],
    [Container1], [#sym.arrow.t], [(100, 100)], [p: (90, 90)], [], [],
)*/