package Grid is

    type Cell_Type is new Integer;

    type Rows is new Natural range 1 .. 9;
    type Columns is new Natural range 1 .. 9;
    type Grid_Type is array (Rows, Columns) of Cell_Type;

    function Initialize return Grid_Type;
end Grid;
