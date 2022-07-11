class Guesstimator
  def self.guess(guesses)
    return nil if guesses.empty?
    new(guesses).guess
  end

  def initialize(guesses)
    @guesses = guesses
  end

  attr_reader :guesses

  def guess
    result = Set.new
    guess_helper \
      result: result,
      work: [nil, nil, nil, nil, nil],
      i: 0,
      state: [],
      dict: Dictionary.instance
    result.to_a.sort
  end

  private

  def guess_helper(result:, work:, i:, state:, dict:)
    if i == work.size
      if dict.terminal? && rules.zip(state).all? { |rule, state| rule.satisfied?(state) }
        result << work.join
      end
      return
    end

    dict.next_letters.each do |l|
      rule_res = rules.zip(state).map { |rule, state| rule.apply(l, position: i, state: state) }
      next unless rule_res.all? { |ok, _| ok }
      work[i] = l
      guess_helper \
        result: result,
        work: work,
        i: i + 1,
        state: rule_res.map { |_, state| state },
        dict: dict.lookup(l)
    end
  end

  def rules
    return @rules if @rules

    @rules = []

    guesses.each do |g|
      no, yes = g.letters.partition { |_, char_type| char_type == Guess::NOT_IN_PUZZLE }
      no = no.map { |l, _| l.downcase }
      yes = yes.map { |l, _| l.downcase }
      (no - yes).each do |l|
        @rules << Rule::NotInPuzzle.new(l)
      end

      counts = Hash.new(0)
      g.letters.each_with_index do |(l, char_type), i|
        case char_type
        when Guess::CORRECT_POSITION
          l = l.downcase
          counts[l] += 1
          @rules << Rule::Match.new(l, position: i)
        when Guess::INCORRECT_POSITION
          l = l.downcase
          counts[l] += 1
          @rules << Rule::Mismatch.new(l, position: i)
        end
      end
      counts.each do |l, count|
        @rules << Rule::Count.new(l, count: count)
      end
    end

    @rules
  end
end
