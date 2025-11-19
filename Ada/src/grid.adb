package body Grid is

    function Initialize return Grid_Type is
        Result : Grid_Type;
    begin
        for R in Rows loop
            for C in Columns loop
                Result (R, C) := 0;
            end loop;
        end loop;
        return Result;
    end Initialize;

end Grid;
