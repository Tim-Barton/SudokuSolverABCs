with Ada.Text_IO;
with Ada.Command_line;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;
with Grid;                  use Grid;

procedure Main is
    Filename : Unbounded_String;
    GridData : Grid_Type;
begin
    Ada.Text_IO.Put_Line ("Starting Sudoku Solver");
    Filename := To_Unbounded_String (Ada.Command_Line.Argument (Number => 1));
    Ada.Text_IO.Put_Line ("Input Data File: " & To_String (Filename));
    GridData := Grid.Initialize;
end Main;
