program read_value
  implicit none

  character(512) :: lines
  integer :: ios

  do
    read(*, *, iostat=ios) lines
    print *, trim(lines)
    if (ios /= 0) then
      return
    end if
  end do

end program read_value
