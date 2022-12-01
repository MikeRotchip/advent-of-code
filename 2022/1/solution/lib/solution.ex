defmodule Solution do

  def main() do
    File.read!("input.txt")
    |> String.split("\n\n")
    |> Enum.map(fn moose -> Enum.reduce(String.split(moose, "\n") , 0, fn x, acc -> String.to_integer(x) + acc end) end)
    |> Enum.sort()
    |> Enum.slice(-3, 3)
    |> Enum.reduce(fn x, acc -> x + acc end)
  end

end
