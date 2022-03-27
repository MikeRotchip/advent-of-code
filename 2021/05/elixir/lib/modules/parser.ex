defmodule Parser do
  def get_line(input) do
    [line_start, line_end] = String.split(input, " -> ")

    [x1, y1] = get_cords(line_start)
    [x2, y2] = get_cords(line_end)

    %{x1: x1, x2: x2, y1: y1, y2: y2}
  end

  def is_not_diagonal(%{x1: x1, x2: x2, y1: y1, y2: y2}) do
    x1 == x2 || y1 == y2
  end

  defp get_cords(input) do
    String.split(input, ",") |> Enum.map(fn x -> String.to_integer(x) end)
  end
end
