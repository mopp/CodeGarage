# -*- coding: utf-8 -*-
require 'gnuplot'

class LabeledVector
    private
        @@id_counter = 0
        @id          = 0
        @vector      = nil
        @label       = 0

    public
    def initialize(vector)
        @vector       = vector
        @id           = @@id_counter
        @@id_counter += 1
    end

    attr_accessor :vector, :label
    attr_reader :id
end


# @brief make AS to feature vector.
# @param k the number of cluster
# @param vectors array of input data
# @return labeled feature vector and each center.
#         label is 0 to k-1
def k_means(k, labeledVectors, loopLimit, isPlusPlus)
    return nil if ((k <= 1) || (labeledVectors == nil) || (labeledVectors.length == 0) ||  (loopLimit <= 0))

    distanceFunc = lambda { |v1, v2|
        return nil if (v1.length != v1.length)

        d   = 0
        v1i = v1.each
        v2i = v2.each
        loop do
            d += ((v1i.next - v2i.next) ** 2)
        end

        return d
    }

    if isPlusPlus == true
        # First cluster is selected ramdomely.
        clusterCenters = [LabeledVector.new(labeledVectors[rand(labeledVectors.length - 1)].vector.clone)]
        clusterCenters[0].label = 0

        isCenter = lambda { |v|
            clusterCenters.each { |cv|
                return true if (v.vector == cv.vector)
            }

            return false
        }

        getMinDistance = lambda { |v|
            minDistance = Float::INFINITY
            clusterCenters.each do |cv|
                d = distanceFunc.call(cv.vector, v)
                minDistance = d if (d < minDistance)
            end

            return minDistance
        }

        # Roulette selection
        for i in 1..(k - 1)
            total = 0.0


            # Compute each minimum distance to any center.
            labeledVectors.each do |lv|
                next if (isCenter.call(lv) == true)

                d = getMinDistance.call(lv.vector)
                total += (d ** 2);
            end
            arrow = Random.rand(total)
            sum = 0.0

            labeledVectors.each do |lv|
                next if (isCenter.call(lv) == true)

                d    = getMinDistance.call(lv.vector)
                sum += (d ** 2)
                if (arrow <= sum)
                    # Select
                    selectedVector       = LabeledVector.new(lv.vector.clone)
                    selectedVector.label = i
                    clusterCenters.push(selectedVector)
                    break
                end
            end
        end
    else
        # Select random center cluster from dataset.
        clusterCenters = Array.new(k)
        randIndex = (0..(labeledVectors.length - 1)).to_a.shuffle!
        for i in 0..(k - 1)
            clusterCenters[i] = LabeledVector.new(labeledVectors[randIndex[i]].vector.clone)
            clusterCenters[i].label = i
        end
    end

    # Classify iteration
    loopCount = 0
    loop do
        breakFlag = true
        loopCount += 1

        # Classify each vector
        for v in labeledVectors
            storedLabel = v.label

            minDistance = Float::INFINITY
            for i in 0..(k - 1)
                # Calculate distance.
                d = distanceFunc.call(clusterCenters[i].vector, v.vector)
                if (d < minDistance)
                    minDistance = d
                    v.label = i
                end
            end

            breakFlag = false if (storedLabel != v.label)
        end

        break if (breakFlag == true || 2000 < loopCount)

        # Calculate center vector by average
        for cv in clusterCenters
            # Clear.
            cv.vector.fill(0.0, 0)
            cnt = 0.0

            # Summation
            for lv in labeledVectors
                next if (cv.label != lv.label)
                cnt += 1.0
                lv.vector.each.with_index(0) do |val, index|
                    cv.vector[index] += val
                end
            end

            cv.vector.map! do |e|
                e /= cnt
            end
        end
    end

    return [loopCount, clusterCenters]
end


# The number of data
N = 500
MAX_RADIUS = 10.0

dataSet    = Array.new(N)
basePoints = [[45, 60], [70, 80], [50, 50], [65, 50]]
correctDataSet = Array.new(basePoints.length).map!{
    [[], []]
}
for i in 0..(N - 1)
    # Generate polar coordinates system.
    r     = rand(MAX_RADIUS) * ((i % 2 == 0) ? 1.0 : 1.5)
    theta = (rand(-180.0..180.0)) * Math::PI / 180.0

    # Convert into euclid space
    baseIdx    = rand(basePoints.length)
    bp         = basePoints[baseIdx]
    x          = bp[0] + (r * Math.cos(theta))
    y          = bp[1] + (r * Math.sin(theta))
    dataSet[i] = LabeledVector.new([x, y])

    correctDataSet[baseIdx][0].push(x)
    correctDataSet[baseIdx][1].push(y)
end

k = 4
r = k_means(k, dataSet, 2000, true)
clusterCenters = r[1]

classifiedData = Array.new(k).map! {
    [[], []]
}
dataSet.each { |v|
    a = classifiedData[v.label]
    a[0].push(v.vector[0])
    a[1].push(v.vector[1])
}

RANGE_MAX = 110.0

# Plot classify result.
Gnuplot.open do |gp|
    Gnuplot::Plot.new( gp ) do | plot |
        plot.xrange "[0:#{RANGE_MAX}]"
        plot.yrange "[0:#{RANGE_MAX}]"
        classifiedData.each.with_index(1) do |v, index|
            plot.data << Gnuplot::DataSet.new( [v[0], v[1]] ) do |ds|
                ds.with = "points pointtype 5 linecolor #{index}"
                ds.notitle
            end
        end
        clusterCenters.each.with_index(5) do |v, index|
            plot.data << Gnuplot::DataSet.new( [[v.vector[0]], [v.vector[1]]] ) do |ds|
                ds.with = "points pointtype 7 linecolor 7"
                ds.notitle
            end
        end
    end
end

# Plot correct answer.
Gnuplot.open do |gp|
    Gnuplot::Plot.new( gp ) do | plot |
        plot.xrange "[0:#{RANGE_MAX}]"
        plot.yrange "[0:#{RANGE_MAX}]"
        correctDataSet.each.with_index(1) do |v, index|
            plot.data << Gnuplot::DataSet.new( [v[0], v[1]] ) do |ds|
                ds.with = "points pointtype 5 linecolor #{index}"
                ds.notitle
            end
        end
    end
end
