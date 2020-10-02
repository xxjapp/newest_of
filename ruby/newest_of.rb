#!/usr/bin/env ruby
# encoding: utf-8
#
# Introduction:
#   get newest modified time of files in a folder
#
# Examples:
#   > ruby newest_of.rb ./
#   > ruby newest_of.rb /tmp
#

require 'find'

################################################################
# methods

module Test
    def self.newest_of dir
        newest, total_count, path = 0, 0, nil

        Find.find dir do |p|
            mtime = File.mtime(p).to_i

            if mtime > newest
                newest = mtime
                path = p
            end

            total_count += 1
        end

        return newest, path, total_count
    end
end

################################################################
# main

if __FILE__ == $0
    newest, path, total_count = Test.newest_of ARGV[0]

    puts "#{Time.at(newest)} #{path}" 
    puts "total count: #{total_count}"
end
