defmodule Main do

  alias Parser
  alias Board

  def main() do
    lines =
      File.read!("lib/input")                             |>
      String.split("\n")                                  |>
      Enum.map(fn x -> Parser.get_line(x) end)            |>
      Enum.filter(fn x -> Parser.is_not_diagonal(x) end)

    add_lines(Board.new(), lines) |> Board.get_crossings()
  end

  defp add_lines(map, [head | tail]) do
    Board.add_line(map, head) |>
    add_lines(tail)
  end

  defp add_lines(map, []) do
    map
  end
end
