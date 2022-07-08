class Dictionary
  def self.parse(path)
    nodes = {}
    root = nil
    File.open(path) do |f|
      while node = parse_node(f, nodes)
        root = node
      end
    end
    new(root)
  end

  def self.parse_node(io, nodes = {})
    data = io.gets(";")
    return nil if data.nil?
    data =~ /^\[(\d+)(!?)\]( (.+))?;$/ or raise "bad node: #{data.inspect}"
    id = $1.to_i
    terminal = $2 == "!"
    data = $4
    letters = [nil]*26
    if data
      data.split(" ").each do |edge|
        pos, child_id = edge.split(":").map(&:to_i)
        letters[pos] = nodes.fetch(child_id)
      end
    end
    node = Node.new \
      id: id,
      terminal: terminal,
      letters: letters
    nodes[id] = node
    node
  end

  def initialize(root)
    @root = root
  end

  def inspect
    "#<Dictionary>"
  end

  def has_word?(word)
    node = @root
    word.chars.each do |c|
      node = node.lookup(c) or return false
    end
    node.terminal?
  end

  class Node
    def initialize(id:, terminal:, letters:)
      @id = id
      @terminal = terminal
      @letters = letters
    end

    def terminal?
      @terminal
    end

    def lookup(c)
      @letters[ctop(c)]
    end

    private

    A = 'a'.ord

    def ctop(c)
      c.downcase.ord - A
    end
  end
end
