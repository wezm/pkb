module CustomElements

  def self.process!(doc)
    elements.each { |element| element.process!(doc) }
  end

protected

  def self.elements
    [CustomElements::RecentlyChangedList]
  end

end
