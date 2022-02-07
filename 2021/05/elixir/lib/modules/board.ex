defmodule Board do

  @board_size 999

  def new() do
    for x <- 0 .. @board_size, y <- 0 .. @board_size, into: %{}, do: {format_key(%{x: x, y: y}), 0}
  end

  def add_line(map, %{x1: x1, y1: y1, x2: x2, y2: y2}) do
    points = for x <- x1 .. x2, y <- y1 .. y2, do: %{:x => x, :y => y}
    add_points(map, points)
  end

  def get_crossings(map) do
    get_all_cords() |>
    Enum.reduce(0,
      fn cords, x ->
        if get(map, cords) > 1 do
            x + 1
          else
            x
        end
    end)
  end

  defp add_points(map, [head | tail]) do
    increment_rank(map, head) |>
    add_points(tail)
  end

  defp add_points(map, []) do
    map
  end

  defp increment_rank(map, cords) do
    existing_val = get(map, cords)

    set(map, cords, existing_val + 1)
  end

  defp get(map, cords) do
    map[format_key(cords)]
  end

  defp set(map, cords, val) do
    %{map | format_key(cords) => val}
  end

  defp format_key (%{x: x, y: y}) do
    ~s"#{x}-#{y}"
  end

  defp get_all_cords() do
    for x <- 0 .. @board_size, y <- 0 .. @board_size, do: %{x: x, y: y}
  end
end
